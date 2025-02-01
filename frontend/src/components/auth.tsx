import { ReactNode } from "react";

import { useAuth } from "@/context/auth";
import { Navigate } from "react-router-dom";

export const ProtectedRoute = ({ children }: { children: ReactNode }) => {
  const { loading, accessToken } = useAuth();
  if (loading) {
    return <div>Loading...</div>;
  }
  return accessToken ? children : <Navigate to="/auth/login" />;
};
