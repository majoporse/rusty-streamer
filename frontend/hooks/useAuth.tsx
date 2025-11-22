// AuthContainer.ts

import { useCallback, useEffect, useState } from "react";
import { createContainer } from "unstated-next";
import AuthEvents from "@/lib/AuthEvents";
import { useToken } from "./useToken";
import { useRouter } from "next/navigation";

export interface UserBase {
  name: string;
  email: string;
  password: string;
}

export interface User extends UserBase {
  _id: string;
  role: "user" | "admin";
}

// stolen from this article
// https://blog.galmalachi.com/react-and-jwt-authentication-the-right-way

function useAuth() {
  const router = useRouter();
  const [user, setUser] = useState<User | null>(null);
  const refreshToken = useCallback(refresh, []);

  const onTokenInvalid = () => setUser(null);
  const { setToken, clearToken, isAuthenticated } = useToken(
    onTokenInvalid,
    refreshToken
  );

  useEffect(() => {
    // try to get new token on first render using refresh token
    refreshToken();
  }, [refreshToken]);

  useEffect(() => {
    // add listener for login or logout from other tabs
    window.addEventListener(
      "storage",
      async (event: WindowEventMap["storage"]) => {
        if (event.key === AuthEvents.LOGOUT && isAuthenticated()) {
          await clearToken(false);
          setUser(null);
        } else if (event.key === AuthEvents.LOGIN) {
          refreshToken();
        }
      }
    );
  }, [clearToken, isAuthenticated, refreshToken]);

  const logout = useCallback(() => {
    clearToken().finally(() => {
      setUser(null);
      router.push("/");

      // fire an event to logout from all tabs
      window.localStorage.setItem(AuthEvents.LOGOUT, new Date().toISOString());
    });
  }, [router, clearToken]);

  const register = useCallback(
    async (userToRegister: UserBase) => {
      const {
        data: { user, ...rest },
      } = await axios.post<UserAndTokenResponse>("register", userToRegister);
      setUser(user);
      setToken(rest);
    },
    [setToken]
  );

  const login = useCallback(
    async (email: string, password: string) => {
      const {
        data: { user, ...rest },
      } = await axios.post<UserAndTokenResponse>("login", {
        email,
        password,
      });
      setUser(user);
      setToken(rest);

      // fire an event to let all tabs know they should login
      window.localStorage.setItem(AuthEvents.LOGIN, new Date().toISOString());
    },
    [setToken]
  );

  async function refresh() {
    const {
      data: { user, ...rest },
    } = await axios.get<UserAndTokenResponse>("refresh-token");

    setUser(user);
    setToken(rest);
  }

  return {
    user,
    setUser,
    register,
    login,
    logout,
    refreshToken,
  };
}

export const AuthContainer = createContainer(useAuth);
