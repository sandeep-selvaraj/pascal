# pascal info

Print a formatted overview of the workspace.

```
pascal info
```

## Output

```
  Workspace: shop  (python 3.12)
  ├── packages
  │   ├── cart     0.1.0
  │   └── auth     0.1.0
  └── apps
      └── storefront  0.1.0
          ├── depends on: cart
          └── depends on: auth
```

Shows:

- Workspace name and Python version
- All packages with their versions
- All apps with their versions and internal dependency links

## No arguments

`pascal info` takes no arguments or flags.
