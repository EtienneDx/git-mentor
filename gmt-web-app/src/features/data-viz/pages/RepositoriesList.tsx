import { useState, useEffect } from "react";
import RepositoriesAPI from "../services/repositoriesAPI";
import { Repository } from "../../../helpers/types";
import { ColumnHeader } from "../components/ColumnHeader";
import { ColumnContent } from "../components/ColumnContent";

const RepositoriesList = () => {
  const [repositories, setRepositories] = useState<Repository[]>([]);

  useEffect(() => {
    const loadRepositories = async () => {
      const newRepositories: Repository[] = await RepositoriesAPI.fetch();
      setRepositories(newRepositories);
    };
    loadRepositories();
  }, []);

  return (
    <div>
      <h1>My Repositories</h1>
      {repositories.length === 0 ? (
        <h1>Loading Repositories...</h1>
      ) : (
        <div className="flex flex-col justify-stretch">
          <table>
            <tr>
              <ColumnHeader title="Name" />
              <ColumnHeader title="Creation Date" />
            </tr>
            {repositories.map((repo) => (
              <tr key={repo.id}>
                <ColumnContent content={repo.name} />
                <ColumnContent content={repo.creationDate.toDateString()} />
              </tr>
            ))}
          </table>
        </div>
      )}
    </div>
  );
};

export default RepositoriesList;
