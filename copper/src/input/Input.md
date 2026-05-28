# Input

## Overview
The input system handles all raw keyboard and mouse inputs every frame and stores it inside InputState. 
This gives the user with a simple and consistent way to read inputs inside their game.

The system tracks:
- Keyboard states
- Mouse button states
- Mouse position

This is then stored in InputState: Raw input storage


# InputState

## Keyboard Inputs
Keyboard inputs are stored in two sets:
- keys_down: Currently held keys
- keys_pressed: Keys pressed this frame

## Mouse Inputs
Mouse inputs instead are stored in three sets
mouse_buttons_down: Currently pressed mouse buttons
mouse_buttons_pressed: Mouse buttons pressed this frame
mouse_pos: Current position of the cursor (x, y)

## Accessing Input
The input system is available through Resourses, and can be queried using helper functions:
- is_key_down(KeyCode)
- is_key_pressed(KeyCode)
- is_mouse_down(MouseButton)
- is_pressed_pressed(MouseButton)

## Example Usage in Engine
```rust
if resourses.input.state.is_key_down(KeyCode::KeyW) {
    //what you want the entity to do
}
```

