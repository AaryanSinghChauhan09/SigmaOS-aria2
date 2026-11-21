# SigmaOS-aria2

SigmaOS integration component for **aria2 download accelerator**.

## Commands

```bash
sigmaos-aria2 install   # install package + deploy defaults
sigmaos-aria2 deploy    # copy config templates to $HOME
sigmaos-aria2 status    # binary/package/config report
sigmaos-aria2 reset     # remove deployed config
sigmaos-aria2 open      # launch the application
```

## Design

- Shell-based component manager (Omarchy-style per-app repos)
- Secure defaults; no secrets stored
- Deterministic `test.sh` with isolated `$HOME`
