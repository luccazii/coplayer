//! Experimental X11/XWayland integration. No desktop screenshot fallback.
use super::WindowTarget;
use serde_json::{Value, json};
use x11rb::{
    connection::Connection,
    protocol::{composite::ConnectionExt as _, xproto::*},
    rust_connection::RustConnection,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn atom(c: &RustConnection, name: &str) -> Result<u32> {
    Ok(c.intern_atom(false, name.as_bytes())?.reply()?.atom)
}
fn property(c: &RustConnection, window: u32, name: &str) -> Result<GetPropertyReply> {
    Ok(
        c.get_property(false, window, atom(c, name)?, AtomEnum::ANY, 0, 4096)?
            .reply()?,
    )
}
fn number(c: &RustConnection, window: u32, name: &str) -> Result<u32> {
    property(c, window, name)?
        .value32()
        .and_then(|mut v| v.next())
        .ok_or_else(|| format!("Propriedade {name} ausente.").into())
}
fn target(c: &RustConnection, id: u32) -> Result<WindowTarget> {
    let pid = number(c, id, "_NET_WM_PID")?;
    if pid == 0 || pid == std::process::id() || pid > i32::MAX as u32 {
        return Err("Janela sem processo elegível.".into());
    }
    let attributes = c.get_window_attributes(id)?.reply()?;
    if attributes.map_state != MapState::VIEWABLE {
        return Err("Janela não visível.".into());
    }
    let geometry = c.get_geometry(id)?.reply()?;
    let name = property(c, id, "_NET_WM_NAME")?;
    let name = if name.value.is_empty() {
        property(c, id, "WM_NAME")?
    } else {
        name
    };
    let class = property(c, id, "WM_CLASS")?;
    Ok(WindowTarget {
        id,
        pid: pid as i32,
        title: String::from_utf8_lossy(&name.value).into_owned(),
        app_name: String::from_utf8_lossy(&class.value)
            .trim_end_matches('\0')
            .replace('\0', " / "),
        width: geometry.width.into(),
        height: geometry.height.into(),
    })
}
fn windows(c: &RustConnection, root: u32) -> Result<Vec<WindowTarget>> {
    let list = property(c, root, "_NET_CLIENT_LIST")?;
    let ids: Vec<_> = list
        .value32()
        .ok_or("O compositor não anuncia a lista de janelas X11.")?
        .collect();
    Ok(ids
        .into_iter()
        .filter_map(|id| target(c, id).ok())
        .collect())
}
fn verify(c: &RustConnection, request: &Value) -> Result<WindowTarget> {
    let id = u32::try_from(request["windowId"].as_u64().ok_or("Janela inválida.")?)?;
    let target = target(c, id)?;
    if Some(i64::from(target.pid)) != request["pid"].as_i64() {
        return Err("A janela mudou de processo. Selecione novamente.".into());
    }
    Ok(target)
}
fn capture(c: &RustConnection, screen: usize, target: &WindowTarget, path: &str) -> Result<()> {
    let version = c.composite_query_version(0, 4)?.reply()?;
    if version.major_version == 0 && version.minor_version < 2 {
        return Err("Composite 0.2 indisponível.".into());
    }
    // Name only an already redirected window: never redirect a game or read the root.
    let pixmap = c.generate_id()?;
    c.composite_name_window_pixmap(target.id, pixmap)?.check()?;
    let geometry = c.get_geometry(pixmap)?.reply()?;
    let width = u32::from(geometry.width);
    let height = u32::from(geometry.height);
    if width == 0 || height == 0 || u64::from(width) * u64::from(height) > 33_554_432 {
        return Err("Dimensões nativas da captura excedem o limite.".into());
    }
    let attributes = c.get_window_attributes(target.id)?.reply()?;
    let visual = c.setup().roots[screen]
        .allowed_depths
        .iter()
        .flat_map(|d| &d.visuals)
        .find(|v| v.visual_id == attributes.visual)
        .ok_or("Visual X11 desconhecido.")?;
    let format = c
        .setup()
        .pixmap_formats
        .iter()
        .find(|f| f.depth == geometry.depth)
        .ok_or("Formato X11 desconhecido.")?;
    if format.bits_per_pixel != 32
        || visual.red_mask != 0xff0000
        || visual.green_mask != 0xff00
        || visual.blue_mask != 0xff
    {
        return Err("Formato de pixels X11 ainda não suportado.".into());
    }
    let reply = c
        .get_image(
            ImageFormat::Z_PIXMAP,
            pixmap,
            0,
            0,
            geometry.width,
            geometry.height,
            u32::MAX,
        )?
        .reply()?;
    let rgb = decode_pixels(
        &reply.data,
        width,
        height,
        c.setup().image_byte_order == ImageOrder::LSB_FIRST,
    )?;
    let image = image::RgbImage::from_raw(width, height, rgb).ok_or("Imagem inválida.")?;
    let image = image::DynamicImage::ImageRgb8(image);
    let image = if width > crate::storage::MAX_DIMENSION || height > crate::storage::MAX_DIMENSION {
        image.resize(
            crate::storage::MAX_DIMENSION,
            crate::storage::MAX_DIMENSION,
            image::imageops::FilterType::Triangle,
        )
    } else {
        image
    };
    image.save_with_format(path, image::ImageFormat::Png)?;
    // Pixmap is released with this short-lived connection, including error paths.
    Ok(())
}
fn decode_pixels(data: &[u8], width: u32, height: u32, little_endian: bool) -> Result<Vec<u8>> {
    let pixels = u64::from(width) * u64::from(height);
    if pixels == 0 || pixels > 33_554_432 || data.len() as u64 != pixels * 4 {
        return Err("Tamanho de pixels inválido.".into());
    }
    let mut output = Vec::with_capacity(pixels as usize * 3);
    for bytes in data.as_chunks::<4>().0 {
        if little_endian {
            output.extend_from_slice(&[bytes[2], bytes[1], bytes[0]]);
        } else {
            output.extend_from_slice(&bytes[1..4]);
        }
    }
    Ok(output)
}
pub fn request(request: Value) -> std::result::Result<Value, String> {
    execute(&request).map_err(|e| format!("Integração X11/XWayland: {e}"))
}
fn execute(request: &Value) -> Result<Value> {
    let connection = x11rb::connect(None);
    if request["action"] == "status" {
        return Ok(
            json!({"supported": connection.is_ok(), "permissionGranted": connection.is_ok()}),
        );
    }
    let (c, screen) = connection?;
    let root = c.setup().roots[screen].root;
    match request["action"].as_str().unwrap_or("") {
        "windows" => Ok(json!({"windows": windows(&c, root)?})),
        "permission" => Ok(json!({})),
        "capture" => {
            let selected = verify(&c, request)?;
            capture(
                &c,
                screen,
                &selected,
                request["path"].as_str().ok_or("Caminho ausente.")?,
            )?;
            Ok(json!({}))
        }
        "activate" => {
            let selected = verify(&c, request)?;
            let active = atom(&c, "_NET_ACTIVE_WINDOW")?;
            let event =
                ClientMessageEvent::new(32, selected.id, active, [2, x11rb::CURRENT_TIME, 0, 0, 0]);
            c.send_event(
                false,
                root,
                EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY,
                event,
            )?
            .check()?;
            c.flush()?;
            for _ in 0..20 {
                if number(&c, root, "_NET_ACTIVE_WINDOW").ok() == Some(selected.id) {
                    return Ok(json!({}));
                }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            Err("O compositor não confirmou o retorno de foco; painel restaurado.".into())
        }
        _ => Err("Operação não implementada.".into()),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "requires an explicitly selected synthetic X11 window"]
    fn captures_explicit_synthetic_window() {
        let id: u32 = std::env::var("COPLAYER_TEST_WINDOW_ID")
            .unwrap()
            .parse()
            .unwrap();
        let path = std::env::var("COPLAYER_TEST_CAPTURE_PATH").unwrap();
        let (connection, screen) = x11rb::connect(None).unwrap();
        let selected = target(&connection, id).unwrap();
        capture(&connection, screen, &selected, &path).unwrap();
        let captured = image::open(path).unwrap();
        assert!(captured.width() > 0 && captured.height() > 0);
    }
    #[test]
    fn pixels_handle_byte_order_and_ignore_alpha() {
        assert_eq!(decode_pixels(&[3, 2, 1, 0], 1, 1, true).unwrap(), [1, 2, 3]);
        assert_eq!(
            decode_pixels(&[0, 1, 2, 3], 1, 1, false).unwrap(),
            [1, 2, 3]
        );
    }
    #[test]
    fn rejects_truncated_and_oversized_images() {
        assert!(decode_pixels(&[0; 3], 1, 1, true).is_err());
        assert!(decode_pixels(&[], u32::MAX, u32::MAX, true).is_err());
        assert!(decode_pixels(&[], 0, 0, true).is_err());
    }
}
