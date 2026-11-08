#!/usr/bin/env bash
# Read-only capability inventory. Never collects screenshots or window titles.
set -eu
printf 'Coplayer — diagnóstico de sessão (não comprova suporte)\n'
printf 'Kernel: %s / %s\n' "$(uname -s)" "$(uname -m)"
printf 'Sessão: %s\nDesktop: %s\n' "${XDG_SESSION_TYPE:-desconhecida}" "${XDG_CURRENT_DESKTOP:-desconhecido}"
if command -v pgrep >/dev/null && pgrep -x gamescope >/dev/null; then
  printf 'gamescope: processo presente; confirmar Gaming Mode manualmente\n'
else
  printf 'gamescope: processo não encontrado\n'
fi
if command -v gdbus >/dev/null; then
  if portal=$(timeout 10s gdbus introspect --session --dest org.freedesktop.portal.Desktop --object-path /org/freedesktop/portal/desktop 2>/dev/null); then
    for capability in Screenshot ScreenCast GlobalShortcuts; do
      if [[ "$portal" == *"interface org.freedesktop.portal.$capability {"* ]]; then
        printf 'Portal %s: anunciado; operação não testada\n' "$capability"
      else
        printf 'Portal %s: não anunciado\n' "$capability"
      fi
    done
  else
    printf 'Portal: indisponível ou tempo de consulta excedido\n'
  fi
else
  printf 'gdbus: indisponível\n'
fi
printf 'Pendentes: overlay sobre jogo, captura limpa, foco, controle e teclado virtual.\n'
