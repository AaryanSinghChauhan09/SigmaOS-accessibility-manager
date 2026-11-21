# SigmaOS-accessibility-manager

SigmaOS integration component for **Accessibility settings**.

## Commands

```bash
sigmaos-accessibility-manager install   # install package + deploy defaults
sigmaos-accessibility-manager deploy    # copy config templates to $HOME
sigmaos-accessibility-manager status    # binary/package/config report
sigmaos-accessibility-manager reset     # remove deployed config
sigmaos-accessibility-manager open      # launch the application
```

## Design

- Shell-based component manager (Omarchy-style per-app repos)
- Secure defaults; no secrets stored
- Deterministic `test.sh` with isolated `$HOME`
