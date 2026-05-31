use serde::{Deserialize, Serialize};

/// 动画类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnimationType {
    /// 进入动画
    Entrance,
    /// 强调动画
    Emphasis,
    /// 退出动画
    Exit,
    /// 动作路径
    MotionPath,
}

/// 过渡效果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransitionEffect {
    /// 无过渡
    None,
    /// 淡入
    Fade,
    /// 推入
    Push,
    /// 擦除
    Wipe,
    /// 分割
    Split,
    /// 溶解
    Dissolve,
    /// 形状
    Morph,
    /// 缩放
    Zoom,
    /// 旋转
    Rotate,
    /// 百叶窗
    Blinds,
    /// 棋盘
    Checkerboard,
    /// 自定义
    Custom(String),
}

/// 过渡速度
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransitionSpeed {
    /// 非常慢
    VerySlow,
    /// 慢
    Slow,
    /// 中等
    Medium,
    /// 快
    Fast,
    /// 非常快
    VeryFast,
}

impl TransitionSpeed {
    /// 获取持续时间（秒）
    pub fn duration(&self) -> f64 {
        match self {
            TransitionSpeed::VerySlow => 3.0,
            TransitionSpeed::Slow => 2.0,
            TransitionSpeed::Medium => 1.0,
            TransitionSpeed::Fast => 0.5,
            TransitionSpeed::VeryFast => 0.25,
        }
    }
}

/// 动画
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    /// 动画 ID
    pub id: String,
    /// 动画类型
    pub animation_type: AnimationType,
    /// 动画效果名称
    pub effect_name: String,
    /// 目标元素 ID
    pub target_id: String,
    /// 延迟（秒）
    pub delay: f64,
    /// 持续时间（秒）
    pub duration: f64,
    /// 是否自动开始
    pub auto_start: bool,
    /// 触发方式
    pub trigger: String,
}

impl Animation {
    /// 创建新的动画
    pub fn new(
        id: String,
        animation_type: AnimationType,
        effect_name: String,
        target_id: String,
    ) -> Self {
        Self {
            id,
            animation_type,
            effect_name,
            target_id,
            delay: 0.0,
            duration: 1.0,
            auto_start: false,
            trigger: "onClick".to_string(),
        }
    }

    /// 设置延迟
    pub fn with_delay(mut self, delay: f64) -> Self {
        self.delay = delay;
        self
    }

    /// 设置持续时间
    pub fn with_duration(mut self, duration: f64) -> Self {
        self.duration = duration;
        self
    }

    /// 设置自动开始
    pub fn with_auto_start(mut self, auto: bool) -> Self {
        self.auto_start = auto;
        self
    }

    /// 设置触发方式
    pub fn with_trigger(mut self, trigger: String) -> Self {
        self.trigger = trigger;
        self
    }

    /// 创建淡入动画
    pub fn fade_in(id: String, target_id: String) -> Self {
        Self::new(id, AnimationType::Entrance, "fadeIn".to_string(), target_id)
    }

    /// 创建淡出动画
    pub fn fade_out(id: String, target_id: String) -> Self {
        Self::new(id, AnimationType::Exit, "fadeOut".to_string(), target_id)
    }

    /// 创建缩放动画
    pub fn zoom(id: String, target_id: String) -> Self {
        Self::new(id, AnimationType::Entrance, "zoom".to_string(), target_id)
    }

    /// 创建旋转动画
    pub fn spin(id: String, target_id: String) -> Self {
        Self::new(id, AnimationType::Emphasis, "spin".to_string(), target_id)
    }
}

/// 幻灯片过渡
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlideTransition {
    /// 过渡效果
    pub effect: TransitionEffect,
    /// 过渡速度
    pub speed: TransitionSpeed,
    /// 是否应用过渡
    pub enabled: bool,
}

impl SlideTransition {
    /// 创建新的过渡
    pub fn new(effect: TransitionEffect, speed: TransitionSpeed) -> Self {
        Self {
            effect,
            speed,
            enabled: true,
        }
    }

    /// 设置是否启用
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// 创建淡入过渡
    pub fn fade() -> Self {
        Self::new(TransitionEffect::Fade, TransitionSpeed::Medium)
    }

    /// 创建推入过渡
    pub fn push() -> Self {
        Self::new(TransitionEffect::Push, TransitionSpeed::Medium)
    }

    /// 创建无过渡
    pub fn none() -> Self {
        Self::new(TransitionEffect::None, TransitionSpeed::Medium).with_enabled(false)
    }
}

impl Default for SlideTransition {
    fn default() -> Self {
        Self::none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transition_speed_duration() {
        assert_eq!(TransitionSpeed::VerySlow.duration(), 3.0);
        assert_eq!(TransitionSpeed::Medium.duration(), 1.0);
        assert_eq!(TransitionSpeed::VeryFast.duration(), 0.25);
    }

    #[test]
    fn test_animation_new() {
        let animation = Animation::new(
            "1".to_string(),
            AnimationType::Entrance,
            "fadeIn".to_string(),
            "target1".to_string(),
        );
        assert_eq!(animation.animation_type, AnimationType::Entrance);
        assert_eq!(animation.target_id, "target1");
    }

    #[test]
    fn test_animation_with_delay() {
        let animation = Animation::new(
            "1".to_string(),
            AnimationType::Entrance,
            "fadeIn".to_string(),
            "target1".to_string(),
        )
        .with_delay(0.5);
        assert_eq!(animation.delay, 0.5);
    }

    #[test]
    fn test_animation_fade_in() {
        let animation = Animation::fade_in("1".to_string(), "target1".to_string());
        assert_eq!(animation.effect_name, "fadeIn");
        assert_eq!(animation.animation_type, AnimationType::Entrance);
    }

    #[test]
    fn test_animation_fade_out() {
        let animation = Animation::fade_out("1".to_string(), "target1".to_string());
        assert_eq!(animation.effect_name, "fadeOut");
        assert_eq!(animation.animation_type, AnimationType::Exit);
    }

    #[test]
    fn test_animation_zoom() {
        let animation = Animation::zoom("1".to_string(), "target1".to_string());
        assert_eq!(animation.effect_name, "zoom");
    }

    #[test]
    fn test_animation_spin() {
        let animation = Animation::spin("1".to_string(), "target1".to_string());
        assert_eq!(animation.effect_name, "spin");
        assert_eq!(animation.animation_type, AnimationType::Emphasis);
    }

    #[test]
    fn test_animation_chaining() {
        let animation = Animation::fade_in("1".to_string(), "target1".to_string())
            .with_delay(0.5)
            .with_duration(1.5)
            .with_auto_start(true);
        assert_eq!(animation.delay, 0.5);
        assert_eq!(animation.duration, 1.5);
        assert!(animation.auto_start);
    }

    #[test]
    fn test_slide_transition_new() {
        let transition = SlideTransition::new(TransitionEffect::Fade, TransitionSpeed::Medium);
        assert_eq!(transition.effect, TransitionEffect::Fade);
        assert!(transition.enabled);
    }

    #[test]
    fn test_slide_transition_with_enabled() {
        let transition = SlideTransition::new(TransitionEffect::Fade, TransitionSpeed::Medium)
            .with_enabled(false);
        assert!(!transition.enabled);
    }

    #[test]
    fn test_slide_transition_fade() {
        let transition = SlideTransition::fade();
        assert_eq!(transition.effect, TransitionEffect::Fade);
    }

    #[test]
    fn test_slide_transition_push() {
        let transition = SlideTransition::push();
        assert_eq!(transition.effect, TransitionEffect::Push);
    }

    #[test]
    fn test_slide_transition_none() {
        let transition = SlideTransition::none();
        assert!(!transition.enabled);
    }

    #[test]
    fn test_slide_transition_default() {
        let transition = SlideTransition::default();
        assert!(!transition.enabled);
    }

    #[test]
    fn test_animation_serialization() {
        let animation = Animation::fade_in("1".to_string(), "target1".to_string());
        let json = serde_json::to_string(&animation);
        assert!(json.is_ok());
    }

    #[test]
    fn test_slide_transition_serialization() {
        let transition = SlideTransition::fade();
        let json = serde_json::to_string(&transition);
        assert!(json.is_ok());
    }
}
