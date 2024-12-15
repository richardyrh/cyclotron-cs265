use std::convert::Into;
use std::sync::Arc;
use crate::base::behavior::*;

pub struct ComponentBase<T, C> {
    pub cycle: u64,
    pub frequency: u64,
    pub state: T,
    pub config: Parameters<Arc<C>>,
}

impl<T: Default, C: Default> Default for ComponentBase<T, C> {
    fn default() -> Self {
        Self {
            cycle: 0,
            frequency: 500 << 20,
            state: T::default(),
            config: Parameters::default(),
        }
    }
}

pub trait IsComponent: ComponentBehaviors {
    type StateType;
    type ConfigType;

    fn new(config: &Self::ConfigType) -> Self;

    fn base(&mut self) -> &mut ComponentBase<Self::StateType, Self::ConfigType>;

    fn base_ref(&self) -> &ComponentBase<Self::StateType, Self::ConfigType>;

    fn state(&mut self) -> &mut Self::StateType{
        &mut self.base().state
    }

    fn state_ref(&self) -> &Self::StateType {
        &self.base_ref().state
    }

    /// get all children, parameterizable or not
    fn get_children(&mut self) -> Vec<&mut dyn ComponentBehaviors> {
        // Vec::<&mut dyn ComponentBehaviors>::
        vec![]
    }

    // get only parameterizable children
    fn get_param_children(&mut self) -> Vec<&mut dyn Parameterizable<ConfigType=Self::ConfigType>> {
        vec![]
    }

}

impl<X> Parameterizable for X where X: IsComponent {
    type ConfigType = X::ConfigType;

    fn conf(&self) -> &Self::ConfigType {
        self.base_ref().config.c.get().unwrap()
    }

    fn init_conf(&mut self, conf: Arc<Self::ConfigType>) {
        self.get_param_children().iter_mut().for_each(|c| {
            c.init_conf(conf.clone());
        });
        self.base().config.c.set(conf.clone()).map_err(|_| "config already set").unwrap();
    }
}

macro_rules! component_inner {
    ($T:ty, $C:ty) => {
        type StateType = $T;
        type ConfigType = $C;

        fn base(&mut self) -> &mut ComponentBase<$T, $C> {
            &mut self.base
        }

        fn base_ref(&self) -> &ComponentBase<$T, $C> {
            &self.base
        }
    };
}

pub(crate) use component_inner;

/// arguments: identifier, state type, config type, additional methods
macro_rules! component {
    ($comp:ident, $T:ty, $C:ty, $($method:item)*) => {
        impl IsComponent for $comp {
            type StateType = $T;
            type ConfigType = $C;

            fn base(&mut self) -> &mut ComponentBase<$T, $C> {
                &mut self.base
            }

            fn base_ref(&self) -> &ComponentBase<$T, $C> {
                &self.base
            }

            $($method)*
        }
    };
}

pub(crate) use component;
