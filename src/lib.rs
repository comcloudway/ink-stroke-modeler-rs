//! Unofficial Rust bindings for the [ink-stroke-modeler](https://github.com/google/ink-stroke-modeler) C++ library. Using [autocxx](https://crates.io/crates/autocxx).

// Since we can't silence the lint for the Input_EventType type generated by autocxx, we need to set it here
#![allow(clippy::enum_variant_names)]

use autocxx::prelude::*;

use crate::ffi::ink::stroke_model;

autocxx::include_cpp! {
    #include "ink_stroke_modeler/types.h"
    #include "ink_stroke_modeler/params.h"
    #include "ink_stroke_modeler/stroke_modeler.h"
    #include "extras.h"

    safety!(unsafe)

    // types.h
    generate_pod!("ink::stroke_model::Vec2")
    generate_pod!("ink::stroke_model::Input_EventType")
    generate!("ink::stroke_model::Input")
    generate!("ink::stroke_model::Result")
    // stroke_modeler.h
    generate!("ink::stroke_model::StrokeModeler")

    // extras
    generate_pod!("BdWobbleSmootherParams")
    generate_pod!("BdPositionModelerParams")
    generate_pod!("BdSamplingParams")
    generate_pod!("BdStylusStateModelerParams")
    //generate_pod!("BdStrokeEndPredictorParams")
    generate_pod!("BdKalmanPredictorConfidenceParams")
    generate_pod!("BdKalmanPredictorParams")
    //generate!("BdPredictionParams")
    generate!("BdStrokeModelParams")

    generate!("bd_stroke_model_params_new_w_stroke_end_predictor")
    generate!("bd_stroke_model_params_new_w_kalman_predictor")

    generate!("stroke_modeler_new")
    generate!("stroke_modeler_reset")
    generate!("stroke_modeler_reset_w_params")
    generate!("stroke_modeler_update")
    generate!("stroke_modeler_predict")

    generate!("input_new")
    generate!("input_get_event_type")
    generate!("input_get_position")
    generate!("input_get_time")
    generate!("input_get_pressure")
    generate!("input_get_tilt")
    generate!("input_get_orientation")

    generate!("result_make_unique")
    generate!("result_get_position")
    generate!("result_get_velocity")
    generate!("result_get_time")
    generate!("result_get_pressure")
    generate!("result_get_tilt")
    generate!("result_get_orientation")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
pub enum ModelerInputEventType {
    kDown,
    kMove,
    kUp,
}

impl From<stroke_model::Input_EventType> for ModelerInputEventType {
    fn from(t: stroke_model::Input_EventType) -> Self {
        match t {
            stroke_model::Input_EventType::kDown => Self::kDown,
            stroke_model::Input_EventType::kMove => Self::kMove,
            stroke_model::Input_EventType::kUp => Self::kUp,
        }
    }
}

impl From<ModelerInputEventType> for stroke_model::Input_EventType {
    fn from(t: ModelerInputEventType) -> Self {
        match t {
            ModelerInputEventType::kDown => Self::kDown,
            ModelerInputEventType::kMove => Self::kMove,
            ModelerInputEventType::kUp => Self::kUp,
        }
    }
}

pub struct ModelerInput(cxx::UniquePtr<stroke_model::Input>);

impl std::fmt::Display for ModelerInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "input: {{ event_type: {:?}, pos: {:?}, time: {}, pressure: {}, tilt: {}, orientation: {} }}",
            self.get_event_type(),
            self.get_pos(),
            self.get_time(),
            self.get_pressure(),
            self.get_tilt(),
            self.get_orientation(),
        )
    }
}

impl From<cxx::UniquePtr<stroke_model::Input>> for ModelerInput {
    fn from(i: cxx::UniquePtr<stroke_model::Input>) -> Self {
        Self(i)
    }
}

impl ModelerInput {
    pub fn new(
        event_type: ModelerInputEventType,
        pos: (f32, f32),
        time: f64,
        pressure: f32,
        tilt: f32,
        orientation: f32,
    ) -> Self {
        Self(
            crate::ffi::input_new(
                event_type.into(),
                stroke_model::Vec2 { x: pos.0, y: pos.1 },
                time,
                pressure,
                tilt,
                orientation,
            )
            .within_unique_ptr(),
        )
    }

    fn into_ffi(self) -> cxx::UniquePtr<stroke_model::Input> {
        self.0
    }

    pub fn get_event_type(&self) -> ModelerInputEventType {
        ModelerInputEventType::from(crate::ffi::input_get_event_type(&self.0))
    }

    pub fn get_pos(&self) -> (f32, f32) {
        let pos = crate::ffi::input_get_position(&self.0);

        (pos.x, pos.y)
    }

    pub fn get_time(&self) -> f64 {
        crate::ffi::input_get_time(&self.0)
    }

    pub fn get_pressure(&self) -> f32 {
        crate::ffi::input_get_pressure(&self.0)
    }

    pub fn get_tilt(&self) -> f32 {
        crate::ffi::input_get_tilt(&self.0)
    }

    pub fn get_orientation(&self) -> f32 {
        crate::ffi::input_get_orientation(&self.0)
    }
}

pub struct ModelerResult(cxx::UniquePtr<crate::ffi::ink::stroke_model::Result>);

impl From<cxx::UniquePtr<stroke_model::Result>> for ModelerResult {
    fn from(r: cxx::UniquePtr<stroke_model::Result>) -> Self {
        Self(r)
    }
}

impl std::fmt::Display for ModelerResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "res: {{ pos: {:?}, velocity: {:?}, time: {}, pressure: {}, tilt: {}, orientation: {} }}",
            self.get_pos(),
            self.get_velocity(),
            self.get_time(),
            self.get_pressure(),
            self.get_tilt(),
            self.get_orientation(),
        )
    }
}

impl ModelerResult {
    #[allow(unused)]
    fn into_ffi(self) -> cxx::UniquePtr<stroke_model::Result> {
        self.0
    }

    pub fn get_pos(&self) -> (f32, f32) {
        let pos = crate::ffi::result_get_position(&self.0);

        (pos.x, pos.y)
    }

    pub fn get_velocity(&self) -> (f32, f32) {
        let velocity = crate::ffi::result_get_velocity(&self.0);

        (velocity.x, velocity.y)
    }

    pub fn get_time(&self) -> f64 {
        crate::ffi::result_get_time(&self.0)
    }

    pub fn get_pressure(&self) -> f32 {
        crate::ffi::result_get_pressure(&self.0)
    }

    pub fn get_tilt(&self) -> f32 {
        crate::ffi::result_get_tilt(&self.0)
    }

    pub fn get_orientation(&self) -> f32 {
        crate::ffi::result_get_orientation(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[allow(unused)]
pub enum PredictionParams {
    StrokeEnd,
    Kalman(KalmanPredictorParams),
}

impl PredictionParams {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct KalmanPredictorParams {
    pub process_noise: f64,
    pub measurement_noise: f64,
    pub min_stable_iteration: i32,
    pub max_time_samples: i32,
    pub min_catchup_velocity: f32,
    pub acceleration_weight: f32,
    pub jerk_weight: f32,
    pub prediction_interval: f64,
    pub confidence_desired_number_of_samples: i32,
    pub confidence_max_estimation_distance: f32,
    pub confidence_min_travel_speed: f32,
    pub confidence_max_travel_speed: f32,
    pub confidence_max_linear_deviation: f32,
    pub confidence_baseline_linearity_confidence: f32,
}

impl KalmanPredictorParams {
    pub fn suggested() -> Self {
        Self {
            process_noise: 1.0,
            measurement_noise: 1.0,
            min_stable_iteration: 4,
            max_time_samples: 20,
            min_catchup_velocity: 0.02,
            acceleration_weight: 0.5,
            jerk_weight: 0.1,
            prediction_interval: 0.02,
            confidence_desired_number_of_samples: 20,
            confidence_max_estimation_distance: 1.5,
            confidence_min_travel_speed: 1.0,
            confidence_max_travel_speed: 5.0,
            confidence_max_linear_deviation: 10.0,
            confidence_baseline_linearity_confidence: 0.4,
        }
    }

    fn into_ffi(self) -> crate::ffi::BdKalmanPredictorParams {
        let confidence_params = crate::ffi::BdKalmanPredictorConfidenceParams {
            desired_number_of_samples: self.confidence_desired_number_of_samples,
            max_estimation_distance: self.confidence_max_estimation_distance,
            min_travel_speed: self.confidence_min_travel_speed,
            max_travel_speed: self.confidence_max_travel_speed,
            max_linear_deviation: self.confidence_max_linear_deviation,
            baseline_linearity_confidence: self.confidence_baseline_linearity_confidence,
        };

        crate::ffi::BdKalmanPredictorParams {
            process_noise: self.process_noise,
            measurement_noise: self.measurement_noise,
            min_stable_iteration: self.min_stable_iteration,
            max_time_samples: self.max_time_samples,
            min_catchup_velocity: self.min_catchup_velocity,
            acceleration_weight: self.acceleration_weight,
            jerk_weight: self.jerk_weight,
            prediction_interval: self.prediction_interval,
            confidence_params,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct StrokeModelerParams {
    pub wobble_smoother_timeout: f64,
    pub wobble_smoother_speed_floor: f32,
    pub wobble_smoother_speed_ceiling: f32,
    pub position_modeler_spring_mass_constant: f32,
    pub position_modeler_drag_constant: f32,
    pub sampling_min_output_rate: f64,
    pub sampling_end_of_stroke_stopping_distance: f32,
    pub sampling_end_of_stroke_max_iterations: i32,
    pub sampling_max_outputs_per_call: i32,
    pub stylus_state_modeler_max_input_samples: i32,
    pub prediction_params: PredictionParams,
}

impl StrokeModelerParams {
    pub fn suggested() -> Self {
        Self {
            wobble_smoother_timeout: 0.04,
            wobble_smoother_speed_floor: 1.31,
            wobble_smoother_speed_ceiling: 1.44,
            position_modeler_spring_mass_constant: 11.0 / 32400.0,
            position_modeler_drag_constant: 72.0,
            sampling_min_output_rate: 180.0,
            sampling_end_of_stroke_stopping_distance: 0.001,
            sampling_end_of_stroke_max_iterations: 20,
            sampling_max_outputs_per_call: 20,
            stylus_state_modeler_max_input_samples: 100_000,
            prediction_params: PredictionParams::StrokeEnd,
        }
    }

    fn into_ffi(self) -> cxx::UniquePtr<crate::ffi::BdStrokeModelParams> {
        let wobble_smoother_params = crate::ffi::BdWobbleSmootherParams {
            timeout: self.wobble_smoother_timeout,
            speed_floor: self.wobble_smoother_speed_floor,
            speed_ceiling: self.wobble_smoother_speed_ceiling,
        };

        let position_modeler_params = crate::ffi::BdPositionModelerParams {
            spring_mass_constant: self.position_modeler_spring_mass_constant,
            drag_constant: self.position_modeler_drag_constant,
        };

        let sampling_params = crate::ffi::BdSamplingParams {
            min_output_rate: self.sampling_min_output_rate,
            end_of_stroke_stopping_distance: self.sampling_end_of_stroke_stopping_distance,
            end_of_stroke_max_iterations: self.sampling_end_of_stroke_max_iterations,
            max_outputs_per_call: self.sampling_max_outputs_per_call,
        };

        let stylus_state_params = crate::ffi::BdStylusStateModelerParams {
            max_input_samples: self.stylus_state_modeler_max_input_samples,
        };

        match self.prediction_params {
            PredictionParams::StrokeEnd => {
                crate::ffi::bd_stroke_model_params_new_w_stroke_end_predictor(
                    wobble_smoother_params,
                    position_modeler_params,
                    sampling_params,
                    stylus_state_params,
                )
                .within_unique_ptr()
            }
            PredictionParams::Kalman(kalman_params) => {
                crate::ffi::bd_stroke_model_params_new_w_kalman_predictor(
                    wobble_smoother_params,
                    position_modeler_params,
                    sampling_params,
                    stylus_state_params,
                    kalman_params.into_ffi(),
                )
                .within_unique_ptr()
            }
        }
    }
}

pub struct StrokeModeler(cxx::UniquePtr<stroke_model::StrokeModeler>);

impl Default for StrokeModeler {
    fn default() -> Self {
        let params = StrokeModelerParams::suggested();

        Self(crate::ffi::stroke_modeler_new(params.into_ffi()).within_unique_ptr())
    }
}

impl StrokeModeler {
    pub fn new(params: StrokeModelerParams) -> Self {
        Self(crate::ffi::stroke_modeler_new(params.into_ffi()).within_unique_ptr())
    }

    pub fn reset(&mut self) {
        crate::ffi::stroke_modeler_reset(self.0.pin_mut());
    }

    pub fn reset_w_params(&mut self, params: StrokeModelerParams) {
        crate::ffi::stroke_modeler_reset_w_params(self.0.pin_mut(), params.into_ffi());
    }

    pub fn update(&mut self, input: ModelerInput) -> Vec<ModelerResult> {
        let results = crate::ffi::stroke_modeler_update(self.0.pin_mut(), input.into_ffi());

        results
            .into_iter()
            .map(|r| ModelerResult::from(crate::ffi::result_make_unique(r)))
            .collect()
    }

    pub fn predict(&mut self) -> Vec<ModelerResult> {
        let results = crate::ffi::stroke_modeler_predict(self.0.pin_mut());

        results
            .into_iter()
            .map(|r| ModelerResult::from(crate::ffi::result_make_unique(r)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn feed_input() {
        let inputs = vec![
            crate::ModelerInput::new(
                crate::ModelerInputEventType::kDown,
                (0.0, 0.0),
                0.0,
                0.1,
                0.0,
                0.0,
            ),
            crate::ModelerInput::new(
                crate::ModelerInputEventType::kMove,
                (1.0, 0.0),
                0.02,
                0.3,
                0.0,
                0.0,
            ),
            crate::ModelerInput::new(
                crate::ModelerInputEventType::kMove,
                (2.0, 0.0),
                0.04,
                0.5,
                0.0,
                0.0,
            ),
            crate::ModelerInput::new(
                crate::ModelerInputEventType::kMove,
                (2.5, 1.0),
                0.06,
                0.8,
                0.0,
                0.0,
            ),
            crate::ModelerInput::new(
                crate::ModelerInputEventType::kMove,
                (3.0, 1.5),
                0.12,
                0.9,
                0.0,
                0.0,
            ),
            crate::ModelerInput::new(
                crate::ModelerInputEventType::kMove,
                (4.0, 2.0),
                0.13,
                0.8,
                0.0,
                0.0,
            ),
            crate::ModelerInput::new(
                crate::ModelerInputEventType::kMove,
                (3.8, 2.1),
                0.13,
                0.7,
                0.0,
                0.0,
            ),
            crate::ModelerInput::new(
                crate::ModelerInputEventType::kUp,
                (3.5, 2.0),
                0.14,
                0.2,
                0.0,
                0.0,
            ),
        ];
        let mut modeler = crate::StrokeModeler::default();

        for res in inputs.into_iter().flat_map(|i| modeler.update(i)) {
            println!("{res}")
        }
    }

    #[test]
    fn modeler_reset() {
        let mut modeler = crate::StrokeModeler::default();

        modeler.reset();
    }

    #[test]
    fn modeler_reset_w_params() {
        let mut modeler = crate::StrokeModeler::default();

        modeler.reset_w_params(crate::StrokeModelerParams::suggested());
    }
}
