import { Route } from "react-router-dom";
import SignInPage from "../views/auth/SignInPage";
import SignUpPage from "../views/auth/SignUpPage";

export function PublicRoutes() {
  return [
    <Route key="sign-in" path="/sign-in/*" element={<SignInPage />} />,
    <Route key="sign-up" path="/sign-up/*" element={<SignUpPage />} />,
  ];
}
