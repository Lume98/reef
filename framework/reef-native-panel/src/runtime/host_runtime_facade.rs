// 内部门面：按职责重导出宿主运行时子模块能力，减少上层对实现文件的耦合。

pub(crate) use super::host_runtime_command::*;
pub(crate) use super::host_runtime_dispatch::*;
pub(crate) use super::host_runtime_state::*;

#[cfg(test)]
mod tests {
    use super::{
        create_native_panel_via_host_controller, execute_native_panel_runtime_host_command,
        hide_native_panel_via_host_controller, native_panel_host_display_reposition,
        native_panel_host_display_reposition_from_input_descriptor,
        native_panel_runtime_host_reposition_command_from_input_descriptor,
        reposition_native_panel_host_from_input_descriptor_via_controller,
        set_native_panel_host_shared_body_height_via_controller,
        sync_runtime_host_display_reposition_in_state, NativePanelComputedHostWindow,
        NativePanelHostDisplayReposition, NativePanelRuntimeHostCommand,
        NativePanelRuntimeHostController, NativePanelRuntimeHostState,
    };
    use crate::{
        presentation::render::{
            NativePanelHostWindowDescriptor, NativePanelPointerRegion,
            NativePanelRuntimeInputDescriptor,
        },
        scene::PanelSceneBuildInput,
        state::PanelRect,
    };

    #[derive(Default)]
    struct TestRuntimeHostState {
        descriptor: NativePanelHostWindowDescriptor,
        pointer_regions: Vec<NativePanelPointerRegion>,
    }

    impl NativePanelRuntimeHostState for TestRuntimeHostState {
        fn host_window_descriptor_mut(&mut self) -> &mut NativePanelHostWindowDescriptor {
            &mut self.descriptor
        }

        fn pointer_regions_mut(&mut self) -> &mut Vec<NativePanelPointerRegion> {
            &mut self.pointer_regions
        }
    }

    #[derive(Default)]
    struct TestComputedHostWindow {
        descriptor: NativePanelHostWindowDescriptor,
        last_frame: Option<PanelRect>,
        created: bool,
    }

    #[derive(Default)]
    struct TestHostController {
        create_count: usize,
        hide_count: usize,
        last_reposition: Option<NativePanelHostDisplayReposition>,
        last_shared_body_height: Option<f64>,
    }

    impl NativePanelComputedHostWindow for TestComputedHostWindow {
        fn host_window_descriptor(&self) -> NativePanelHostWindowDescriptor {
            self.descriptor
        }

        fn host_window_descriptor_mut(&mut self) -> &mut NativePanelHostWindowDescriptor {
            &mut self.descriptor
        }

        fn host_window_last_frame_mut(&mut self) -> &mut Option<PanelRect> {
            &mut self.last_frame
        }

        fn host_window_frame(&self) -> Option<PanelRect> {
            self.last_frame
        }

        fn set_host_window_created(&mut self) {
            self.created = true;
        }

        fn fallback_screen_frame(&self) -> PanelRect {
            PanelRect {
                x: 0.0,
                y: 0.0,
                width: 1440.0,
                height: 900.0,
            }
        }

        fn compact_width(&self) -> f64 {
            crate::state::DEFAULT_COMPACT_PILL_WIDTH
        }

        fn expanded_width(&self) -> f64 {
            crate::state::DEFAULT_EXPANDED_PILL_WIDTH
        }
    }

    impl NativePanelRuntimeHostController for TestHostController {
        type Error = String;

        fn runtime_host_create_panel(&mut self) -> Result<(), Self::Error> {
            self.create_count += 1;
            Ok(())
        }

        fn runtime_host_hide_panel(&mut self) -> Result<(), Self::Error> {
            self.hide_count += 1;
            Ok(())
        }

        fn runtime_host_reposition(
            &mut self,
            reposition: NativePanelHostDisplayReposition,
        ) -> Result<(), Self::Error> {
            self.last_reposition = Some(reposition);
            Ok(())
        }

        fn runtime_host_set_shared_body_height(
            &mut self,
            body_height: f64,
        ) -> Result<(), Self::Error> {
            self.last_shared_body_height = Some(body_height);
            Ok(())
        }
    }

    #[test]
    fn display_reposition_payload_can_be_built_from_runtime_input_descriptor() {
        let input = NativePanelRuntimeInputDescriptor {
            scene_input: PanelSceneBuildInput {
                settings: crate::state::PanelSettingsState {
                    selected_display_index: 2,
                    ..Default::default()
                },
                ..Default::default()
            },
            screen_frame: Some(PanelRect {
                x: 40.0,
                y: 50.0,
                width: 800.0,
                height: 600.0,
            }),
        };

        assert_eq!(
            native_panel_host_display_reposition_from_input_descriptor(&input),
            NativePanelHostDisplayReposition {
                preferred_display_index: 2,
                screen_frame: Some(PanelRect {
                    x: 40.0,
                    y: 50.0,
                    width: 800.0,
                    height: 600.0,
                }),
            }
        );
    }

    #[test]
    fn runtime_host_reposition_command_can_be_built_from_input_descriptor() {
        let input = NativePanelRuntimeInputDescriptor {
            scene_input: PanelSceneBuildInput {
                settings: crate::state::PanelSettingsState {
                    selected_display_index: 4,
                    ..Default::default()
                },
                ..Default::default()
            },
            screen_frame: Some(PanelRect {
                x: 100.0,
                y: 120.0,
                width: 1024.0,
                height: 768.0,
            }),
        };

        assert_eq!(
            native_panel_runtime_host_reposition_command_from_input_descriptor(&input),
            NativePanelRuntimeHostCommand::Reposition(NativePanelHostDisplayReposition {
                preferred_display_index: 4,
                screen_frame: Some(PanelRect {
                    x: 100.0,
                    y: 120.0,
                    width: 1024.0,
                    height: 768.0,
                }),
            })
        );
    }

    #[test]
    fn runtime_host_state_can_sync_display_reposition_payload() {
        let mut state = TestRuntimeHostState::default();

        sync_runtime_host_display_reposition_in_state(
            &mut state,
            native_panel_host_display_reposition(
                3,
                Some(PanelRect {
                    x: 40.0,
                    y: 50.0,
                    width: 800.0,
                    height: 600.0,
                }),
            ),
        );

        assert_eq!(state.descriptor.preferred_display_index, 3);
        assert_eq!(
            state.descriptor.screen_frame,
            Some(PanelRect {
                x: 40.0,
                y: 50.0,
                width: 800.0,
                height: 600.0,
            })
        );
    }

    #[test]
    fn computed_host_window_reposition_consumes_shared_display_payload() {
        let mut window = TestComputedHostWindow::default();

        window.host_window_reposition_to_display(
            1,
            Some(PanelRect {
                x: 100.0,
                y: 100.0,
                width: 1440.0,
                height: 900.0,
            }),
        );

        assert!(window.created);
        assert_eq!(window.descriptor.preferred_display_index, 1);
        assert_eq!(
            window.descriptor.screen_frame,
            Some(PanelRect {
                x: 100.0,
                y: 100.0,
                width: 1440.0,
                height: 900.0,
            })
        );
    }

    #[test]
    fn host_controller_helpers_route_create_hide_reposition_and_height() {
        let mut controller = TestHostController::default();
        let input = NativePanelRuntimeInputDescriptor {
            scene_input: PanelSceneBuildInput {
                settings: crate::state::PanelSettingsState {
                    selected_display_index: 2,
                    ..Default::default()
                },
                ..Default::default()
            },
            screen_frame: Some(PanelRect {
                x: 10.0,
                y: 20.0,
                width: 800.0,
                height: 600.0,
            }),
        };

        create_native_panel_via_host_controller(&mut controller).expect("create via helper");
        hide_native_panel_via_host_controller(&mut controller).expect("hide via helper");
        reposition_native_panel_host_from_input_descriptor_via_controller(&mut controller, &input)
            .expect("reposition via helper");
        set_native_panel_host_shared_body_height_via_controller(&mut controller, 240.0)
            .expect("shared body height via helper");

        assert_eq!(controller.create_count, 1);
        assert_eq!(controller.hide_count, 1);
        assert_eq!(
            controller.last_reposition,
            Some(NativePanelHostDisplayReposition {
                preferred_display_index: 2,
                screen_frame: Some(PanelRect {
                    x: 10.0,
                    y: 20.0,
                    width: 800.0,
                    height: 600.0,
                }),
            })
        );
        assert_eq!(controller.last_shared_body_height, Some(240.0));
    }

    #[test]
    fn runtime_host_command_executor_routes_all_host_commands() {
        let mut controller = TestHostController::default();
        let reposition = NativePanelHostDisplayReposition {
            preferred_display_index: 5,
            screen_frame: Some(PanelRect {
                x: 1.0,
                y: 2.0,
                width: 3.0,
                height: 4.0,
            }),
        };

        execute_native_panel_runtime_host_command(
            &mut controller,
            NativePanelRuntimeHostCommand::Create,
        )
        .expect("create command");
        execute_native_panel_runtime_host_command(
            &mut controller,
            NativePanelRuntimeHostCommand::Hide,
        )
        .expect("hide command");
        execute_native_panel_runtime_host_command(
            &mut controller,
            NativePanelRuntimeHostCommand::Reposition(reposition),
        )
        .expect("reposition command");
        execute_native_panel_runtime_host_command(
            &mut controller,
            NativePanelRuntimeHostCommand::SetSharedBodyHeight(280.0),
        )
        .expect("height command");

        assert_eq!(controller.create_count, 1);
        assert_eq!(controller.hide_count, 1);
        assert_eq!(controller.last_reposition, Some(reposition));
        assert_eq!(controller.last_shared_body_height, Some(280.0));
    }
}
