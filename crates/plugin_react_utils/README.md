# `@rdil/swc-plugin-react-utils`

Standalone, npm version of [the swc plugin from web-infra-dev](https://github.com/web-infra-dev/swc-plugins/tree/5573d3a8d3ca2793ba31da832244638be3ab008b/crates/plugin_react_utils).

## Options

### `autoImportReact`

Boolean, default `false`.
Automatically appends `import React from "react"` to each file at build time.

### `removeEffect`

Boolean, default `false`.
Removes all `useEffect` calls. Helpful for SSR.

### `removePropTypes`

Boolean, default `false`.
Removes usages of `prop-types`. (e.g. `Component.propTypes = {}` gets stripped out)
