# Application Layout Framework (ALF)

This is a set of UI primitives and components that were borrowed from Bluesky's code base.

I liked the defined and reusable values for consistency for all UI elements

## Atoms (`atoms.ts`)

A set of style definitions that match Tailwind CSS selectors.

These are reused throughout the app

```typescript
import { atoms as a } from '#/alf';

<View style={[a.flex_row]} />
```

## Themes
