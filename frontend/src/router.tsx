import { createBrowserRouter } from "react-router-dom";
import { ProtectedRoute } from '@/components/auth';
import { Root } from '@/components/root';
import { RouteError } from "./components/error";

export const router = createBrowserRouter([
  {
    path: '/auth/login',
    lazy: async () => {
      const { Login } = await import('./pages/auth/login');
      return { Component: Login };
    }
  },
  {
    path: '/',
    element: (
      <ProtectedRoute>
        <Root />
      </ProtectedRoute>
    ),
    errorElement: <RouteError />,
    children: [
      {
        path: '',
        lazy: async () => {
          const { Desktop } = await import('./pages/desktop');
          return { Component: Desktop };
        }
      }
    ]
  },
]);
