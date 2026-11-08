#!/bin/sh
set -eu
package_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
# Keep panel and game integration on the same X11/XWayland server.
export GDK_BACKEND=x11
# Extract-and-run works without installing FUSE or changing SteamOS system files.
exec "$package_dir/Coplayer-Linux-x86_64.AppImage" --appimage-extract-and-run "$@"
