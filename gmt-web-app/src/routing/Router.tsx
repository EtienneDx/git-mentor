import React from "react";
import Authentication from "../features/auth/Authentication";
import { Route, Routes } from "react-router-dom";
import StudentsRouter from "./StudentsRouter";
import RepositoriesRouter from "./RepositoriesRouter";
import GroupsRouter from "./GroupsRouter";

const Router: React.FC = () => {
  return (
    <Routes>
      <Route path="/" element={<Authentication />} />
      <Route path="/groups/*" element={<GroupsRouter />} />
      <Route path="/students/*" element={<StudentsRouter />} />
      <Route path="/repositories/*" element={<RepositoriesRouter />} />
    </Routes>
  );
};

export default Router;
