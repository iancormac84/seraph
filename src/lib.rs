pub mod core;
pub mod platform;

pub use platform::{generic, windows};

pub static mut PUMPING_MESSAGE_OUTSIDE_OF_MAIN_LOOP: bool = true;

/** Thread ID of the main/game thread */
//pub static uint32 GAME_THREAD_ID: u32 = ;

/** Thread ID of the render thread, if any */
//extern CORE_API uint32 GRenderThreadId;

/** Thread ID of the slate thread, if any */
//extern CORE_API uint32 GSlateLoadingThreadId;

/** @return True if called from the game thread. */
pub fn is_in_game_thread() -> bool {
    /*if(GIsGameThreadIdInitialized)
    {
        const uint32 CurrentThreadId = FPlatformTLS::GetCurrentThreadId();
        return CurrentThreadId == GGameThreadId || CurrentThreadId == GSlateLoadingThreadId;
    }*/

    true
}

//The code to initialize GAME_THREAD_ID is in E:\Devel\study\UnrealEngine\Engine\Source\Runtime\Launch\Private\LaunchEngineLoop.cpp.
//This is done by a call to FPlatformTLS::GetCurrentThreadId(). Read this file; it is interesting.
//Also please note that in E:\Devel\study\UnrealEngine\Engine\Source\Runtime\Slate\Private\Framework\Application\SlateApplication.cpp (line 1589?), it is asserted
//that IsInGameThread encompasses two threads in actuality. These are the main thread and the slate loading thread. The code does a check as to whether the GAME_THREAD_ID
//is the same as the thread id returned by a call to FPlatformTLS::GetCurrentThreadId().

//use uuid::{IID_IDropTarget, IID_IUnknown};

//TODO: make all the static mut 'variables' Atomic
