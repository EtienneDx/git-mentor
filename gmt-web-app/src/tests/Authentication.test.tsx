import { render } from "@testing-library/react";
import "@testing-library/jest-dom";
import Login from "../features/auth/Login";
import CombinedContextProvider from "../context";
import { BrowserRouter } from "react-router-dom";
import Signup from "../features/auth/Signup";

jest.mock("axios");

describe("Authentication Component", () => {
  it("Login renders without crashing", () => {
    render(
      <CombinedContextProvider>
        <BrowserRouter>
          <Login />
        </BrowserRouter>
      </CombinedContextProvider>
    );
  });

  it("Signup renders without crashing", () => {
    render(
      <CombinedContextProvider>
        <BrowserRouter>
          <Signup />
        </BrowserRouter>
      </CombinedContextProvider>
    );
  });
});
