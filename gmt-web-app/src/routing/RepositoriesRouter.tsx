import { Route, Routes } from "react-router-dom";
import RepositoriesList from "../features/data-viz/pages/RepositoriesList";

const RepositoriesRouter = () => {
  return (
    <Routes >
      <Route index Component={RepositoriesList} />
    </ Routes>
  );
};

export default RepositoriesRouter;
