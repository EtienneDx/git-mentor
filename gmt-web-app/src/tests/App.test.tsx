import React from "react";
import { render, screen } from "@testing-library/react";
import App from "../App";

describe("App Component", () => {
  it("renders without crashing", () => {
    render(<App />);
  });

  it("renders the Authentication component", () => {
    render(<App />);
    expect(screen.getByText(/login/i)).toBeInTheDocument();
  });
});
