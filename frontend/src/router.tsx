import { createBrowserRouter } from "react-router-dom";
import { ProtectedRoute, RedirectIfLoggedIn } from "@/components/auth";
import { Root } from "@/components/root";
import { RouteError } from "./components/error";
import { NavigationHeader } from "./components/heading";

export const router = createBrowserRouter([
  {
    path: "/auth",
    element: (
      <>
        <NavigationHeader />
        <Root />
      </>
    ),
    children: [
      {
        path: "totp/validate",
        lazy: async () => {
          const { TotpForm } = await import("./pages/auth/totp-validate");
          return { Component: TotpForm };
        },
      },
      {
        path: "totp/login",
        lazy: async () => {
          const { TotpForm } = await import("./pages/auth/totp-login");
          return { Component: TotpForm };
        },
      },
      {
        path: "login",
        lazy: async () => {
          const { LoginForm } = await import("./pages/auth/login");
          return { Component: LoginForm };
        },
      },
      {
        path: "signup",
        lazy: async () => {
          const { SignupForm } = await import("./pages/auth/signup");
          return { Component: SignupForm };
        },
      },
    ],
  },
  {
    path: "/",
    element: (
      <ProtectedRoute>
        <NavigationHeader />
        <Root />
      </ProtectedRoute>
    ),
    errorElement: <RouteError />,
    children: [
      {
        path: "",
        lazy: async () => {
          const { Home } = await import("./pages/admin/home");
          return { Component: Home };
        },
      },
    ],
  },
]);
