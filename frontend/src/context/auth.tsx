import {
  createContext,
  useContext,
  useEffect,
  useMemo,
  useState,
  useRef,
  useCallback,
  useLayoutEffect,
} from "react";
import axios, { AxiosInstance, InternalAxiosRequestConfig } from "axios";
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
};

type AuthActions = {
  login: (email: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
};

type AuthContextValue = AuthState &
  AuthActions & {
    apiClient: AxiosInstance;
  };

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [state, setState] = useState<AuthState>({
    accessToken: null,
    ctx: null,
  });

  const accessTokenRef = useRef(state.accessToken);
  useEffect(() => {
    accessTokenRef.current = state.accessToken;
  }, [state.accessToken]);

  // Create base API client without auth interceptors
  const baseApiClient = useMemo(
    () =>
      axios.create({
        baseURL: import.meta.env.VITE_API_URL,
        withCredentials: true,
        headers: {
          "Content-Type": "application/json",
        },
      }),
    [],
  );

  const refreshToken = useCallback(async (): Promise<boolean> => {
    try {
      const { data } = await baseApiClient.get("/auth/refresh");
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
  }, [baseApiClient]);

  const refreshTokenRef = useRef(refreshToken);
  useEffect(() => {
    refreshTokenRef.current = refreshToken;
  }, [refreshToken]);

  // Create authenticated API client with interceptors
  const authApiClient = useMemo(() => {
    const client = axios.create({
      baseURL: import.meta.env.VITE_API_URL,
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

          const refreshSuccess = await refreshTokenRef.current();
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
    await baseApiClient.post("/auth/logout");
  }, [baseApiClient]);

  const login = useCallback(
    async (email: string, password: string) => {
      try {
        const { data } = await baseApiClient.post(
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
    [baseApiClient],
  );

  // Initial token check
  useLayoutEffect(() => {
    refreshToken();
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

export function useAuth(): AuthContextValue {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
}
