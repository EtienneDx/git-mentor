const RepositoriesAPI = {
  fetch: async () => {
    return [
      {
        name: "Repository A",
        id: "1",
        creationDate: new Date(2000, 1, 1),
      },
      {
        name: "Repository B",
        id: "2",
        creationDate: new Date(2000, 1, 1),
      },
      {
        name: "Repository C",
        id: "3",
        creationDate: new Date(2000, 1, 1),
      },
    ];
  },
};

export default RepositoriesAPI;
