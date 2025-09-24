import { ReactNode } from "react";

import { useAuth } from "@/hooks/auth";
import { Navigate } from "react-router-dom";

export const ProtectedRoute = ({ children }: { children: ReactNode }) => {
  const { ctx } = useAuth();
  return ctx ? children : <Navigate to="/auth/login" />;
};

export const RedirectIfLoggedIn = ({ children }: { children: ReactNode }) => {
  const { ctx } = useAuth();
  return !ctx ? children : <Navigate to="/" />;
};
