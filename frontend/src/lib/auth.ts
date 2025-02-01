import axios from "axios";

let accessToken: string | null = null;

export const getAccessToken = () => accessToken;
export const setAccessToken = (token: string) => {
  accessToken = token;
};
export const clearAccessToken = () => {
  accessToken = null;
};
export const tokenExist = () => {
  return accessToken !== null;
};
export const refreshToken = async () => {
  const data = await axios.get("/auth/refresh", {
    baseURL: import.meta.env.VITE_API_URL,
    withCredentials: true,
  });
  if (data.status == 200) {
    setAccessToken(data.data.jwt);
  }
  return tokenExist();
};
