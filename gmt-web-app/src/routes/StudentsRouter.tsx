import { Route, Routes } from "react-router-dom";
import StudentsList from "../features/data-viz/pages/StudentsList";

const StudentsRouter = () => {
  return (
    <Routes>
      <Route path="/" element={<StudentsList/>} />
    </Routes>
  );
};

export default StudentsRouter;
