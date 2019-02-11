const deleteCommand = async ({ pin }, _, id) => {
	const pinToDelete = pin(id);

	await pinToDelete.delete();
};

export default deleteCommand;
