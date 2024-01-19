use leptos::*;

#[component]
pub fn PasswordForm() -> impl IntoView {
    view! {
        <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
            <div class="sm:mx-auto sm:w-full sm:max-w-sm">
                <img
                    class="mx-auto h-10 w-auto"
                    src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600"
                    alt="Your Company"
                />
                <h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">
                    Unlock password
                </h2>
            </div>

            <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
                <form class="space-y-6" action="#" method="POST">
                    <input
                        type="text"
                        placeholder="Enter key"
                        class="input input-bordered w-full "
                    />

                    <button class="btn btn-primary w-full">Unlock</button>
                </form>

                <p class="mt-10 text-center text-sm text-gray-500">
                    Need to share a password?
                    <a
                        href="#"
                        class="font-semibold leading-6 text-indigo-600 hover:text-indigo-500"
                    >
                        Create one
                    </a>
                </p>
            </div>
        </div>
    }
}
