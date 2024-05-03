import { useState, useEffect } from "react";
import StudentsAPI from "../services/studentsAPI";
import { Student } from "../../../helpers/types";
import { ColumnContent } from "../components/ColumnContent";
import { ColumnHeader } from "../components/ColumnHeader";

const StudentsList = () => {
  const [students, setStudents] = useState<Student[]>([]);

  useEffect(() => {
    const loadStudents = async () => {
      const newStudents: Student[] = await StudentsAPI.fetch();
      setStudents(newStudents);
    };
    loadStudents();
  }, []);

  return (
    <div>
      <h1>My Students</h1>
      {students.length === 0 ? (
        <h1>Loading Students...</h1>
      ) : (
        <div className="flex flex-col justify-stretch">
          <table>
            <tr>
              <ColumnHeader title="Name" />
              <ColumnHeader title="Creation Date" />
            </tr>
            {students.map((stud) => (
              <tr key={stud.id}>
                <ColumnContent content={stud.name} />
                <ColumnContent content={stud.creationDate.toDateString()} />
              </tr>
            ))}
          </table>
        </div>
      )}
    </div>
  );
};

export default StudentsList;
