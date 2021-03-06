/// Dom events
#[derive(Debug, Copy, Clone)]
pub enum Event {
    Abort,
    AfterPrint,
    AnimationEnd,
    AnimationIteration,
    AnimationStart,
    BeforePrint,
    BeforeUnload,
    Blur,
    CanPlay,
    CanPlayThrough,
    Change,
    Click,
    ContextMenu,
    Copy,
    Cut,
    DoubleClick,
    Drag,
    DragEnd,
    DragEnter,
    DragLeave,
    DragOver,
    DragStart,
    Drop,
    DurationChange,
    Ended,
    Error,
    Focus,
    FocusIn,
    FocusOut,
    FullscreenChange,
    FullscreenError,
    HashChange,
    Input,
    Invalid,
    KeyDown,
    KeyPress,
    KeyUp,
    Load,
    LoadedData,
    LoadedMetadata,
    LoadStart,
    Message,
    MouseDown,
    MouseEnter,
    MouseLeave,
    MouseMove,
    MouseOver,
    MouseOut,
    MouseUp,
    MouseWheel,
    Offline,
    Online,
    Open,
    PageHide,
    PageShow,
    Paste,
    Pause,
    Play,
    Playing,
    PopState,
    Progress,
    RateChange,
    Resize,
    Reset,
    Scroll,
    Search,
    Seeked,
    Seeking,
    Select,
    Show,
    Stalled,
    Storage,
    Submit,
    Suspend,
    TimeUpdate,
    Toggle,
    TouchCancel,
    TouchEnd,
    TouchMove,
    TouchStart,
    Transitionend,
    Unload,
    VolumeChange,
    Waiting,
    Wheel,
}

impl Event {
    pub fn to_str(self) -> String {
        format!("{:?}", self).to_ascii_lowercase()
    }
}
