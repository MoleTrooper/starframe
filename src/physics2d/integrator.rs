use super::Velocity;
use crate::util::Transform;

pub enum IntegratorState {
    NeedsDerivatives,
    Done,
}

/// A time stepper for equations of motion, used with physics solvers.
/// Can have multiple substeps and request the solver for derivatives in between.
pub trait Integrator {
    /// Create an integrator in its initial state.
    fn begin_step(timestep: f32) -> Self;

    /// Execute a step of the integration algorithm.
    /// Returns NeedsDerivatives if the next step requires solving constraints,
    /// Done if the step is complete.
    fn substep<'a>(
        &mut self,
        variables: impl Iterator<Item = (&'a mut Transform, &'a mut Velocity)>,
    ) -> IntegratorState;
}

/// Explicit Euler integrator.
/// Uses velocity and position at the start of a step to determine
/// position at the end of a step.
/// Unconditionally unstable, should generally not be used.
pub struct ExplicitEuler {
    timestep: f32,
    done: bool,
}

impl Integrator for ExplicitEuler {
    fn begin_step(timestep: f32) -> Self {
        ExplicitEuler {
            timestep,
            done: false,
        }
    }

    fn substep<'a>(
        &mut self,
        variables: impl Iterator<Item = (&'a mut Transform, &'a mut Velocity)>,
    ) -> IntegratorState {
        if self.done {
            return IntegratorState::Done;
        }

        for (tr, vel) in variables {
            tr.translate(self.timestep * vel.linear);
            tr.rotate_rad(self.timestep * vel.angular);
        }

        self.done = true;
        IntegratorState::NeedsDerivatives
    }
}
