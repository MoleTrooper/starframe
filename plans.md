Everything I've thought of doing but haven't gotten around to yet is here.

**ECS**
- more storage types (see specs)
- automatic object pooling API
- optional components in ComponentFilters
    - investigate using these instead of event listeners
- no crash on unset recipe var
- reconsider Space builder syntax (use `cascade` instead?)
- preset recipes for common objects (can use as template for more specific stuff)
- LockedAnyMap wrapper type to tidy up the syntax for Space-global state (AnyMap with everything RwLocked)

**physics**
- collider types: ~~circle~~, rect, polygon
- rigid body constraint solver
- use temporal coherence as heuristic to optimize collision detection
- spatial partitioning (probably hierarchical grid)
- joints
- some form of fluid simulation (SPH, PBF, something mesh-based??)

**misc**
- add loading level from file to the project template
- try making an actual game with multiple levels, see how the
  design scales (level loading from MES? game state management
  between loading, playing and paused? etc etc)

**open questions**
- additions to MES format?
- some UI framework (conrod / imgui?) or write my own?
- graphics library? Piston feels too limited
    - probably going with glium for now,
      but also interested in vulkan