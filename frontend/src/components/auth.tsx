import { ReactNode } from "react";

import { useAuth } from "@/context/auth";

export const ProtectedRoute = ({ children }: { children: ReactNode }) => {
  const { accessToken } = useAuth();
  console.log(accessToken);
  return accessToken ? children : <div>Oops</div>;
};
