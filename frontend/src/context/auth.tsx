import {
  createContext,
  useEffect,
  useMemo,
  useState,
  Dispatch,
  useRef,
  useCallback,
  useLayoutEffect,
  SetStateAction,
} from "react";
import axios, { AxiosInstance, InternalAxiosRequestConfig } from "axios";
import { UUID } from "crypto";
import { getBaseUrl } from "@/lib/service";

export interface AuthCtx {
  user_id: UUID;
  roles: string[];
  iac: number;
  exp: number;
}

export type AuthState = {
  accessToken: string | null;
  ctx: AuthCtx | null;
};

export type AuthActions = {
  login: (email: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
};

type AuthContextValue = AuthState &
  AuthActions & {
    apiClient: AxiosInstance;
  };

export const AuthContext = createContext<AuthContextValue | null>(null);

const refreshToken = async (
  client: AxiosInstance,
  setState: Dispatch<SetStateAction<AuthState>>,
): Promise<boolean> => {
  try {
    const { data } = await client.get("/auth/refresh");
    setState((prev) => ({
      ...prev,
      accessToken: data.jwt,
      ctx: data.ctx,
    }));
    return true;
  } catch (error) {
    setState((prev) => ({
      ...prev,
      accessToken: null,
      ctx: null,
    }));
    return false;
  }
};

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [state, setState] = useState<AuthState>({
    accessToken: null,
    ctx: null,
  });

  const accessTokenRef = useRef(state.accessToken);
  useEffect(() => {
    accessTokenRef.current = state.accessToken;
  }, [state.accessToken]);

  // Create authenticated API client with interceptors
  const authApiClient = useMemo(() => {
    const client = axios.create({
      baseURL: getBaseUrl("http"),
      withCredentials: true,
    });

    client.interceptors.request.use((config: InternalAxiosRequestConfig) => {
      if (accessTokenRef.current) {
        config.headers.Authorization = `Bearer ${accessTokenRef.current}`;
      }
      return config;
    });

    client.interceptors.response.use(
      (response) => response,
      async (error) => {
        const originalRequest = error.config;
        if (error.response?.status === 401 && !originalRequest._retry) {
          originalRequest._retry = true;

          const refreshSuccess = await refreshToken(client, setState);
          if (!refreshSuccess) {
            await logout();
            return Promise.reject(error);
          }

          originalRequest.headers.Authorization = `Bearer ${accessTokenRef.current}`;
          return client(originalRequest);
        }
        return Promise.reject(error);
      },
    );

    return client;
  }, []);

  const logout = useCallback(async () => {
    setState({
      accessToken: null,
      ctx: null,
    });
    await authApiClient.post("/auth/logout");
  }, [authApiClient]);

  const login = useCallback(
    async (email: string, password: string) => {
      try {
        const { data } = await authApiClient.post(
          "/auth/login",
          { email, password },
          { headers: { "Content-Type": "application/x-www-form-urlencoded" } },
        );

        setState({
          accessToken: data.jwt,
          ctx: data.ctx,
        });
      } catch (error) {
        throw error;
      }
    },
    [authApiClient],
  );

  // Initial token check
  useLayoutEffect(() => {
    refreshToken(authApiClient, setState);
  }, [refreshToken]);

  const value = useMemo(
    () => ({
      ...state,
      login,
      logout,
      apiClient: authApiClient,
    }),
    [state, login, logout, authApiClient],
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}
