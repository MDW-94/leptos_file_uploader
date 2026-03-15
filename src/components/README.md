# Components

To make our application modular and reusable, we will create reusable components, like icons and buttons.

## Icon Components

Our application requires icon components to represent some UI elements visually. We will create an icons module within the components module to organize these icons, using SVG format. You can utilize icons iconoir, heroicons or any other preferred library if desired.

To organize our icons, create the following structure within the components directory.

```
.
├── src
│   ├── components
│   │   ├── icons
│   │   │   │── check_icon.rs
│   │   │   │── info_icon.rs
│   │   │   │── mod.rs
│   │   │   │── spin_icon.rs
│   │   │   └── trash_icon.rs
│   │   └── mod.rs
```

## Check icon component

...
