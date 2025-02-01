import { createContext, useContext, useEffect, useMemo, useState } from "react";
import { apiClient } from "@/lib/axios";
import { UUID } from "crypto";

type AuthState = {
  accessToken: string | null;
  ctx: {
    user_id: UUID;
    roles: string[];
    iac: number;
    exp: number;
    valid_totp: boolean;
  } | null;
  loading: boolean;
};

type AuthActions = {
  logout: () => Promise<void>;
  refreshToken: () => Promise<void>;
};

type AuthContextValue = AuthState & AuthActions;

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [state, setState] = useState<AuthState>({
    accessToken: null,
    ctx: null,
    loading: true,
  });

  const actions = useMemo<AuthActions>(
    () => ({
      logout: async () => {
        setState({
          accessToken: null,
          ctx: null,
          loading: false,
        });
        await apiClient.get("/auth/logout");
        apiClient.defaults.headers.common["Authorization"] = "";
        window.location.href = "/auth/login";
      },

      refreshToken: async () => {
        try {
          const { data } = await apiClient.get("/auth/refresh");
          setState((prev) => ({
            ...prev,
            accessToken: data.accessToken,
            user: data.user,
            loading: false,
          }));
          apiClient.defaults.headers.common["Authorization"] =
            `Bearer ${data.accessToken}`;
        } catch (error) {
          setState((prev) => ({
            ...prev,
            accessToken: null,
            user: null,
            loading: false,
          }));
        }
      },
    }),
    [],
  );

  // Immediate refresh check on mount
  useEffect(() => {
    let mounted = true;

    const initializeAuth = async () => {
      try {
        await actions.refreshToken();
      } finally {
        if (mounted) {
          setState((prev) => ({ ...prev, loading: false }));
        }
      }
    };

    initializeAuth();

    return () => {
      mounted = false;
    };
  }, [actions]);

  const value = useMemo(
    () => ({
      ...state,
      ...actions,
    }),
    [state, actions],
  );

  return (
    <AuthContext.Provider value={value}>
      {state.loading ? (
        <div className="flex h-screen items-center justify-center">
          <span>Loading...</span>
        </div>
      ) : (
        children
      )}
    </AuthContext.Provider>
  );
}

export function useAuth(): AuthContextValue {
  const context = useContext(AuthContext);

  // Explicit error for missing provider
  if (typeof context === "undefined" || context == null) {
    throw new Error("useAuth must be used within an AuthProvider");
  }

  // Now TypeScript knows context is AuthContextValue
  return context;
}
