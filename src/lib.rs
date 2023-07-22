pub mod wrapper;
pub mod error;
pub mod events;
mod value;

mod test {

    use crate::wrapper::{EventWrapper, channel::ComparatorChannel};

    use super::events::*;

    #[allow(dead_code)]
    #[derive(EventCollection, Clone, Debug, PartialEq)]
    pub enum MyEvent {
        ToolbarWidth(f32),
        SettingsWidth(f32),
        LayerValue(u32),
        TimeValue(f32),
    }

    #[test]
    fn event_collection() {
        let event = MyEvent::LayerValue(5);
        
        assert!(event.event_eq_type(MyEventType::LayerValue));
        assert!(!event.event_eq_type(MyEventType::SettingsWidth));

        assert!(MyEvent::ToolbarWidth(2.0).event_eq_type(MyEventType::ToolbarWidth));
        assert!(MyEvent::TimeValue(2.0).event_eq_type(MyEventType::TimeValue));
        assert!(!MyEvent::TimeValue(2.0).event_eq_type(MyEventType::ToolbarWidth));
    }

    #[test]
    fn event_wrapper() -> Result<(), crate::error::RegisterError> {
        let mut channel_list = Vec::<ComparatorChannel<MyEvent>>::new();
        channel_list.push(ComparatorChannel::new(MyEvent::LayerValue(2)));
        channel_list.push(ComparatorChannel::new(MyEvent::SettingsWidth(10.0)));
        channel_list.push(ComparatorChannel::new(MyEvent::ToolbarWidth(15.0)));

        let mut wrapper = EventWrapper::new(channel_list);

        assert!(wrapper.find_channel(MyEventType::SettingsWidth).is_some());
        assert!(wrapper.find_channel(MyEventType::TimeValue).is_none());
        assert_eq!(wrapper.get_channel_value(MyEventType::SettingsWidth), Some(&MyEvent::SettingsWidth(10.0)));
        assert_ne!(wrapper.get_channel_value(MyEventType::SettingsWidth), Some(&MyEvent::SettingsWidth(5.0)));
        assert_ne!(wrapper.get_channel_value(MyEventType::LayerValue), Some(&MyEvent::SettingsWidth(2.0)));
        assert_eq!(wrapper.get_channel_value(MyEventType::LayerValue), Some(&MyEvent::LayerValue(2)));
        assert_eq!(wrapper.get_channel_value(MyEventType::ToolbarWidth), Some(&MyEvent::ToolbarWidth(15.0)));

        wrapper.register_safely(MyEventType::LayerValue, MyEvent::LayerValue(100))?;
        
        assert_ne!(wrapper.get_channel_value(MyEventType::LayerValue), Some(&MyEvent::LayerValue(2)));
        assert_eq!(wrapper.get_channel_value(MyEventType::LayerValue), Some(&MyEvent::LayerValue(100)));
        assert!(wrapper.find_channel(MyEventType::LayerValue).unwrap().has_changed());
        
        wrapper.register_safely(MyEventType::SettingsWidth, MyEvent::SettingsWidth(300.0))?;
        
        assert_ne!(wrapper.get_channel_value(MyEventType::SettingsWidth), Some(&MyEvent::SettingsWidth(2.0)));
        assert_ne!(wrapper.get_channel_value(MyEventType::SettingsWidth), Some(&MyEvent::SettingsWidth(100.0)));
        assert_eq!(wrapper.get_channel_value(MyEventType::SettingsWidth), Some(&MyEvent::SettingsWidth(300.0)));
        assert!(wrapper.find_channel(MyEventType::SettingsWidth).unwrap().has_changed());

        let channel_value = wrapper.register_safely_with_ref(MyEventType::ToolbarWidth, MyEvent::ToolbarWidth(2.0));

        if let Ok(MyEvent::ToolbarWidth(width)) = channel_value {
            *width = 120.0;
        } else {
            assert!(false);
        }

        assert_eq!(wrapper.get_channel_value(MyEventType::ToolbarWidth), Some(&MyEvent::ToolbarWidth(120.0)));
        assert!(wrapper.find_channel(MyEventType::ToolbarWidth).unwrap().has_changed());
        assert_eq!(wrapper.get_channel_value(MyEventType::ToolbarWidth), Some(&MyEvent::ToolbarWidth(120.0)));
        assert!(wrapper.find_channel(MyEventType::ToolbarWidth).unwrap().has_changed());

        Ok(())
    }

}