# `@hypercortex/hypercortex-object`

> TODO: description

## Usage

```js
   const taskSpecification = createHyperCortexObject({
      type: "task",
      calculateScore: task => ( /* do stuff */ 0 ),
      properties: {
         scalars: [
         "description",
         "priority",
         ]
         collections: [
            {
               name: "tags",
               sortBy: tag => tag,
            }
         ],
      }
      relations: {
         one: [
            {
               name: "properties",
               resolver: propertiesSpecification,
            }
         ],
         many: [
            {
               name: "dependsOn",
                     resolver: taskSpecification,
            }
         ]
      }
   });

   const { task, taskAll } = taskSpecification(db);

   const foo = task("id")
   await foo.descriptionSet("this is the description")
   const description = await foo.descriptionGet()
   const calculatedScore = await foo.scoreGet();

   const tasks = await taskAll();
   const foo = tasks[0]
```
