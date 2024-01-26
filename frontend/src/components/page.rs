use leptos::*;
use leptos_router::Outlet;

use crate::{
	components::{Header, Sidebar},
	contexts::ThemeCtx,
};

#[component]
pub fn Page() -> impl IntoView {
	let theme = expect_context::<ThemeCtx>().0 .0;

	let expanded = create_rw_signal(true.to_string());
	provide_context(expanded);

	view! {
		<div
			data-theme=theme
			class="flex flex-row min-h-screen max-h-screen bg-base-100 min-w-screen max-w-screen"
		>
			<Sidebar/>
			<div data-expanded=expanded class=PAGE_CLASSES>
				<Header/>
				<div class="min-h-[calc(100vh-4rem)] max-h-[calc(100vh-4rem)] min-w-screen max-w-screen bg-base-100 overflow-y-auto no-scrollbar">
					<Outlet/>
				</div>
			</div>
		</div>
	}
}

// #[component]
// pub fn ScrollablePage(children: Children) -> impl IntoView {
//     let theme = expect_context::<ThemeCtx>().0 .0;

//     let expanded = create_rw_signal(true.to_string());
//     provide_context(expanded);

//     view! {
//         <div data-theme=theme class="flex flex-row min-h-screen max-h-screen bg-base-100">
//             <Sidebar/>
//             <div data-expanded=expanded class=PAGE_CLASSES>
//                 <Header/>
//                 <div class="min-h-[calc(100vh-4rem)] max-h-[calc(100vh-4rem)] bg-base-100
// overflow-y-scroll">                     {children()}
//                 </div>
//             </div>
//         </div>
//     }
// }

pub const PAGE_CLASSES: &'static str = "
flex flex-col

data-[expanded=false]:w-[calc(100vw-16px)]
data-[expanded=true]:w-[calc(100vw-64px)]
";
