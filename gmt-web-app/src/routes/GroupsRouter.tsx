import { Route, Routes } from "react-router-dom";
import GroupsList from "../features/data-viz/pages/GroupsList";
import GroupAssignements from "../features/data-viz/pages/GroupAssignements";
import GroupStudents from "../features/data-viz/pages/GroupStudents";

const GroupsRouter = () => {
  return (
    <Routes>
      <Route index element={<GroupsList/>} />
      <Route path=":groupId" element={<GroupAssignements/>} />
      <Route path=":groupId/students" element={<GroupStudents/>} />
    </Routes>
  );
};

export default GroupsRouter;
