use std::hashmap::HashMap;
use rendering::{Window, RenderPump};
use event_pump::EventPump;

type StateId = int;

pub enum StateTransition {
	Transition(StateId), //UID of state
	Nil
}

pub trait State {
	fn attach_event_handlers(&mut self, pump: &mut EventPump);
	fn update_renderers(&mut self, pump: &mut RenderPump);
	fn Tick(&mut self, dt: f32) -> StateTransition;
}


pub struct StateHandler {
	states: HashMap<StateId, ~State>,
	currentId: StateId,
	transition: StateTransition
}

impl StateHandler {
	pub fn new (id: StateId, beginState: ~State ) -> StateHandler {
		let mut state = StateHandler { states: HashMap::new(), currentId: id, transition: Nil };
		state.add_state(id, beginState);
		
		state
	}

	pub fn add_state(&mut self, id: StateId, state: ~State) {
		self.states.insert(id, state);
	}

	pub fn attach_event_handlers(&mut self, pump: &mut EventPump) {
		let state = self.get_current_state();
		state.attach_event_handlers(pump)
	}

	pub fn update_renderers(&mut self, pump: &mut RenderPump) {
		let state = self.get_current_state();
		state.update_renderers(pump)
	}

	pub fn Tick(&mut self, dt: f32) {
		let transition = self.get_current_state().Tick(dt);
		self.transition = transition;
		
		match  self.transition {
			Transition(id) => return, 
			_ => return
		}
	}

	fn get_current_state<'a>(&'a mut self) -> &'a mut ~State {
		return self.states.get_mut(&self.currentId)	
	}
}