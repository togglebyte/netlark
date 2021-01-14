
fn main() {
    let mut resources = Resources::default();
    let mut world = World::default();
    let mut schedules = scheds(&mut world, &mut resources);
    let mut state_stack = state_stack();

    for event in events(20) {
        resources.insert(event);
        state_stack.exec(&mut world, &mut resources);
        let transition = pop_next_state(&mut resources);
        // If there is a transition:
        // * state_stack
        // * schedules
        // * and clear renderer
        transition_state(transition, &mut state_stack, &mut schedules);
        resources.get_mut::<Rend>().map(|mut r| r.clear());
    }
}
