#import <AppKit/AppKit.h>
#import <CoreGraphics/CoreGraphics.h>
#import <ScreenCaptureKit/ScreenCaptureKit.h>

// This bridge runs inside Coplayer: capture permission belongs to the application,
// not a command-line helper. Rust calls it only from a blocking worker thread.
static NSDictionary *failure(NSString *message) {
    return @{ @"error": message };
}

static SCShareableContent *content(NSError **resultError) API_AVAILABLE(macos(14.0)) {
    dispatch_semaphore_t done = dispatch_semaphore_create(0);
    __block SCShareableContent *result = nil;
    __block NSError *error = nil;
    [SCShareableContent getShareableContentExcludingDesktopWindows:YES
                                             onScreenWindowsOnly:YES
                                               completionHandler:^(SCShareableContent *value, NSError *cause) {
        result = value;
        error = cause;
        dispatch_semaphore_signal(done);
    }];
    if (dispatch_semaphore_wait(done, dispatch_time(DISPATCH_TIME_NOW, 15 * NSEC_PER_SEC))) {
        *resultError = [NSError errorWithDomain:@"Coplayer" code:1 userInfo:@{NSLocalizedDescriptionKey: @"A listagem de janelas excedeu 15 segundos. Tente novamente."}];
        return nil;
    }
    *resultError = error;
    return result;
}

static NSDictionary *perform(NSDictionary *request) {
    NSString *action = request[@"action"];
    if ([action isEqualToString:@"status"] || [action isEqualToString:@"permission"]) {
        BOOL supported = NO;
        if (@available(macOS 14.0, *)) supported = YES;
        BOOL granted = CGPreflightScreenCaptureAccess();
        if ([action isEqualToString:@"permission"] && !granted) {
            dispatch_sync(dispatch_get_main_queue(), ^{ CGRequestScreenCaptureAccess(); });
            granted = CGPreflightScreenCaptureAccess();
        }
        return @{ @"supported": @(supported), @"permissionGranted": @(granted) };
    }
    if (@available(macOS 14.0, *)) {
        if ([action isEqualToString:@"activate"]) {
            pid_t pid = [request[@"pid"] intValue];
            if (pid <= 0 || pid == getpid()) return failure(@"Selecione uma janela externa ao Coplayer.");
            __block BOOL activated = NO;
            dispatch_sync(dispatch_get_main_queue(), ^{
                NSRunningApplication *target = [NSRunningApplication runningApplicationWithProcessIdentifier:pid];
                if (target && !target.terminated) {
                    activated = [target activateFromApplication:[NSRunningApplication currentApplication] options:0];
                }
            });
            return activated ? @{ @"activated": @YES } : failure(@"O macOS não confirmou a ativação do aplicativo selecionado. Verifique se ele continua aberto.");
        }
        if (!CGPreflightScreenCaptureAccess()) {
            return failure(@"Permita Gravação da Tela para o Coplayer nos Ajustes do Sistema. Se necessário, encerre e abra o aplicativo novamente.");
        }
        NSError *error = nil;
        SCShareableContent *available = content(&error);
        if (!available) return failure(error.localizedDescription ?: @"Não foi possível consultar as janelas.");
        if ([action isEqualToString:@"windows"]) {
            NSMutableArray *windows = [NSMutableArray array];
            for (SCWindow *window in available.windows) {
                SCRunningApplication *owner = window.owningApplication;
                if (!owner || owner.processID == getpid() || !window.onScreen || window.windowLayer != 0 || window.frame.size.width < 80 || window.frame.size.height < 80) continue;
                [windows addObject:@{
                    @"id": @(window.windowID), @"pid": @(owner.processID),
                    @"title": window.title ?: @"Janela sem título", @"appName": owner.applicationName ?: @"Aplicativo",
                    @"width": @(window.frame.size.width), @"height": @(window.frame.size.height)
                }];
            }
            return @{ @"windows": windows };
        }
        if ([action isEqualToString:@"capture"]) {
            SCWindow *selected = nil;
            for (SCWindow *window in available.windows) {
                if (window.windowID == [request[@"windowId"] unsignedIntValue] &&
                    window.owningApplication.processID == [request[@"pid"] intValue] &&
                    window.owningApplication.processID != getpid()) { selected = window; break; }
            }
            if (!selected) return failure(@"A janela selecionada não está mais disponível. Atualize a lista e selecione novamente.");
            SCContentFilter *filter = [[SCContentFilter alloc] initWithDesktopIndependentWindow:selected];
            SCStreamConfiguration *configuration = [SCStreamConfiguration new];
            CGFloat scale = filter.pointPixelScale;
            CGFloat width = selected.frame.size.width * scale;
            CGFloat height = selected.frame.size.height * scale;
            CGFloat reduction = MIN(1.0, [request[@"maxDimension"] doubleValue] / MAX(width, height));
            configuration.width = MAX(1, (size_t)(width * reduction));
            configuration.height = MAX(1, (size_t)(height * reduction));
            configuration.showsCursor = NO;
            configuration.ignoreShadowsSingleWindow = YES;
            dispatch_semaphore_t done = dispatch_semaphore_create(0);
            __block NSData *png = nil;
            __block NSError *captureError = nil;
            [SCScreenshotManager captureImageWithFilter:filter configuration:configuration completionHandler:^(CGImageRef image, NSError *cause) {
                captureError = cause;
                if (image) png = [[[NSBitmapImageRep alloc] initWithCGImage:image] representationUsingType:NSBitmapImageFileTypePNG properties:@{}];
                dispatch_semaphore_signal(done);
            }];
            if (dispatch_semaphore_wait(done, dispatch_time(DISPATCH_TIME_NOW, 15 * NSEC_PER_SEC))) return failure(@"A captura excedeu 15 segundos. Tente novamente.");
            if (!png) return failure(captureError.localizedDescription ?: @"O macOS não retornou uma imagem.");
            if (png.length > [request[@"maxBytes"] unsignedLongLongValue]) return failure(@"A captura excede o limite de 8 MiB.");
            if (![png writeToFile:request[@"path"] options:NSDataWritingAtomic error:&error]) return failure(error.localizedDescription ?: @"Não foi possível salvar a captura local.");
            return @{ @"width": @(configuration.width), @"height": @(configuration.height) };
        }
        return failure(@"Operação de plataforma desconhecida.");
    }
    return failure(@"Esta prova de captura requer macOS 14 ou posterior.");
}

char *coplayer_platform_request(const char *input) {
    @autoreleasepool {
        NSDictionary *request = [NSJSONSerialization JSONObjectWithData:[[NSString stringWithUTF8String:input] dataUsingEncoding:NSUTF8StringEncoding] options:0 error:nil];
        NSDictionary *result = [request isKindOfClass:NSDictionary.class] ? perform(request) : failure(@"Pedido de plataforma inválido.");
        NSData *encoded = [NSJSONSerialization dataWithJSONObject:result options:0 error:nil];
        NSString *json = [[NSString alloc] initWithData:encoded encoding:NSUTF8StringEncoding];
        return strdup(json.UTF8String ?: "{\"error\":\"Falha no bridge nativo.\"}");
    }
}

void coplayer_platform_free(char *value) { free(value); }
