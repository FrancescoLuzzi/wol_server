import axios from "axios";
import { getAccessToken, setAccessToken, clearAccessToken } from "./auth";
// TODO: integrate all this stuff into the auth context manager
// Drop @/lib/auth.ts library

export const apiClient = axios.create({
  baseURL: import.meta.env.VITE_API_URL,
  withCredentials: true,
});

apiClient.interceptors.request.use((config) => {
  const token = getAccessToken();
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

apiClient.interceptors.response.use(
  (response) => response,
  async (error) => {
    const originalRequest = error.config;

    if (error.response?.status === 401 && !originalRequest._retry) {
      originalRequest._retry = true;

      try {
        const { data } = await axios.get("/auth/refresh", {
          baseURL: import.meta.env.VITE_API_URL,
          withCredentials: true,
        });

        setAccessToken(data.jwt);
        return apiClient(originalRequest);
      } catch (refreshError) {
        clearAccessToken();
        window.location.href = "/auth/login";
        return Promise.reject(refreshError);
      }
    }

    return Promise.reject(error);
  },
);
