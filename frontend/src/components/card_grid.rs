use leptos::{component, view, Children, IntoView};

#[component]
pub fn CardGrid(children: Children) -> impl IntoView {
	view! {
		<div class="grid grid-cols-1 gap-x-8 gap-y-12 sm:grid-cols-2 md:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-4">
			{children()}
		</div>
	}
}

#[component]
pub fn GridContainer(children: Children) -> impl IntoView {
	view! {
		<div class="flex flex-col gap-8 justify-center py-4 px-4 w-full sm:px-6 md:px-8 lg:px-12 xl:px-20 2xl:px-28">
			{children()}
		</div>
	}
}
