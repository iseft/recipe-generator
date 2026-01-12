import { Route } from "react-router-dom";
import SignInPage from "./SignInPage";
import SignUpPage from "./SignUpPage";

export function AuthRoutes(): React.ReactElement[] {
  return [
    <Route key="sign-in" path="/sign-in/*" element={<SignInPage />} />,
    <Route key="sign-up" path="/sign-up/*" element={<SignUpPage />} />,
  ];
}
