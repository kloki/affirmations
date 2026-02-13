# Affirmation

Add postivity to your terminal

# Install

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/kloki/affirmations/releases/download/v0.1.1/affirmations-installer.sh | sh
```

## Bash

Add this line to end of you `.bashrc`

```bash
export PS1="$PS1 \$(affirmations)"
```

## Starship

Add this to you starship.toml

```toml
[custom.affirmations]
when = true
command = "affirmations"
format = " $output"
```
