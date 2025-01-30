import { createBrowserRouter } from "react-router-dom";
import { ProtectedRoute } from "@/components/auth";
import { Root } from "@/components/root";
import { RouteError } from "./components/error";

export const router = createBrowserRouter([
  {
    path: "/auth/login",
    lazy: async () => {
      const { LoginForm } = await import("./pages/auth/login");
      return { Component: LoginForm };
    },
  },
  {
    path: "/auth/signup",
    lazy: async () => {
      const { SignupForm } = await import("./pages/auth/signup");
      return { Component: SignupForm };
    },
  },
  {
    path: "/",
    element: (
      <ProtectedRoute>
        <Root />
      </ProtectedRoute>
    ),
    errorElement: <RouteError />,
    children: [
      {
        path: "",
        lazy: async () => {
          const { Home } = await import("./pages/home");
          return { Component: Home };
        },
      },
    ],
  },
]);
