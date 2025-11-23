"use client";

import { useCallback, useEffect, useState } from "react";
import { createContainer } from "unstated-next";
import {
  TokenResponse,
  UserAndTokenResponse,
  useToken,
} from "./useToken";
import { useRouter } from "next/navigation";
import AuthEvents from "@/lib/AuthEvents";
import { AuthApi } from "@/generated";
import { jwtDecode } from "jwt-decode";
import axios from "axios";
import { AxiosConfig } from "@/lib/utils";

export interface UserBase {
  name: string;
  email: string;
}

export interface User extends UserBase {
  _id: string;
  role: "user" | "admin";
}

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
    console.log("Attempting to refresh token on first render");
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
            console.log("Login event detected from another tab");
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
      console.log("Logging in...");
      let api = new AuthApi(AxiosConfig);
      let res = await api.login({ email, password });

      let user = {
        _id: res.data.user_id,
        name: res.data.user_name,
        email: res.data.user_email,
      } as User;

      let decoded = jwtDecode(res.data.access_token) as any;

      console.log("Decoded token:", decoded);

      let tokenResponse: TokenResponse = {
        access_token: res.data.access_token,
        token_expiration: decoded.exp * 1000,
      };

      console.log("Token response:", tokenResponse);

      setUser(user);
      setToken(tokenResponse);

      // fire an event to let all tabs know they should login
      window.localStorage.setItem(AuthEvents.LOGIN, new Date().toISOString());
    },
    [setToken]
  );

  async function refresh() {
    console.log("Refreshing token...");
    let api = new AuthApi(AxiosConfig);
    let res = await api.refresh();

    let user = {
      _id: res.data.user_id,
      name: res.data.user_name,
      email: res.data.user_email,
    } as User;

    let decoded = jwtDecode(res.data.access_token) as any;
    let rest: TokenResponse = {
      access_token: res.data.access_token,
      token_expiration: decoded.exp * 1000,
    };
    console.log("next refresh:", new Date(rest.token_expiration));

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