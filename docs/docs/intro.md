---
sidebar_position: 1
---

# Quick Start Guide

Get started with **Xeeg in less than 5 minutes**.

## Getting Started

Get started by **[installing the binary](/docs/tutorial-basics/installation-guide)**.

### What you'll need

#### Stub Files
Stub files (as used in programming) are placeholders for some functionality which is yet to be defined.

To generate a file from a template, you'll need to first define the template.

The stub files are what Xeeg recognises as templates the files to be generated.

## Defining a template

There's no strict way of generating templates with Xeeg. You can simply define a file which has the file extension of `.stub` (or whatever you choose it to be [refer config docs](/docs/tutorial-basics/configuration))

In this tutorial we'll create an actions (a concept from redux) generator file

```bash
// ./stubs/action.stub

// Generated with Xeeg ;)

import { ActionRequestOptions } from '@lib/types';

const [NEEDLE_CAMEL]Actions = {
	fetch[NEEDLE_PASCAL_PLURAL]: (data?, options?: ActionRequestOptions) => ({
		type: 'FETCH_[NEEDLE_UPPER_SNAKE_PLURAL]',
		data,
		options,
	}),
	fetched[NEEDLE_PASCAL_PLURAL]: (data?) => ({
		type: 'FETCHED_[NEEDLE_UPPER_SNAKE_PLURAL]',
		...data,
	}),
	failedFetch[NEEDLE_PASCAL_PLURAL]: (error?) => ({
		type: 'FAILED_FETCH_[NEEDLE_UPPER_SNAKE_PLURAL]',
		...error,
	}),
	fetch[NEEDLE_PASCAL]: (data?, options?: ActionRequestOptions) => ({
		type: 'FETCH_[NEEDLE_UPPER_SNAKE]',
		data,
		options,
	}),
	fetched[NEEDLE_PASCAL]: (data?) => ({
		type: 'FETCHED_[NEEDLE_UPPER_SNAKE]',
		...data,
	}),
	failedFetch[NEEDLE_PASCAL]: (error?) => ({
		type: 'FAILED_FETCH_[NEEDLE_UPPER_SNAKE]',
		...error,
	}),
	create[NEEDLE_PASCAL]: (payload?) => ({
		type: 'CREATE_[NEEDLE_UPPER_SNAKE]',
		payload: { ...payload, type: '[NEEDLE_SNAKE_PLURAL]' },
		queueable: true,
	}),
	created[NEEDLE_PASCAL]: (data?) => ({
		type: 'CREATED_[NEEDLE_UPPER_SNAKE]',
		...data,
	}),
	failedCreate[NEEDLE_PASCAL]: (error?) => ({
		type: 'FAILED_CREATE_[NEEDLE_UPPER_SNAKE]',
		...error,
	}),
};

export const {
	fetch[NEEDLE_PASCAL_PLURAL],
	fetched[NEEDLE_PASCAL_PLURAL],
	failedFetch[NEEDLE_PASCAL_PLURAL],
	fetch[NEEDLE_PASCAL],
	fetched[NEEDLE_PASCAL],
	failedFetch[NEEDLE_PASCAL],
	create[NEEDLE_PASCAL],
	created[NEEDLE_PASCAL],
	failedCreate[NEEDLE_PASCAL],
} = [NEEDLE_CAMEL]Actions;
```

Notice the use of placeholder strings such as `[NEEDLE_PASCAL_PLURAL]`. These strings are known as needles. Needles get replaced when you generate the end file.

## Generating a file
Finally, we can generate a file from the action template file. To do so, we run the command:

```bash
xeeg make:action ExampleModel example-model-action.ts
```
Breaking down the command:
- ***make:action:*** This command is in two parts separated by a colon (:). The first part, which is `make` tells xeeg that you're trying to generate a file. The second part, `action` is dynamic and is expected to be the name of your stub file (without the extension). Hence Xeeg will attempt to find a stub file called `action.stub` in the stubs directory.

- ***ExampleModel:*** This part is the replacement for all the needle strings in your stub file. Depending on the needle string, the replacement will be formatted accordingly

- ***example-model-action.ts:*** This is the name of the output file we're choosing to save the output into. This file will be placed in the path of the specified output folder

***In summary:*** the command will search for a file in the path `./stubs/action.stub`, replace it's needle strings with the suitably formatted 'ExampleModel' string and finally place the output in the path: `./generated/example-model-action.ts`.

The contents of the output will look like this:

```ts
// ./stubs/action.stub

// Generated with Xeeg ;)

import { ActionRequestOptions } from '@lib/types';

const exampleModelActions = {
	fetchExampleModels: (data?, options?: ActionRequestOptions) => ({
		type: 'FETCH_EXAMPLE_MODELS`,
		data,'		options,
	}),
	fetchedExampleModels: (data?) => ({
		type: 'FETCHED_EXAMPLE_MODELS`,
		...d'ta,
	}),
	failedFetchExampleModels: (error?) => ({
		type: 'FAILED_FETCH_EXAMPLE_MODELS`,
		...e'ror,
	}),
	fetchExampleModel: (data?, options?: ActionRequestOptions) => ({
		type: 'FETCH_EXAMPLE_MODEL`,
		data,'		options,
	}),
	fetchedExampleModel: (data?) => ({
		type: 'FETCHED_EXAMPLE_MODEL`,
		...d'ta,
	}),
	failedFetchExampleModel: (error?) => ({
		type: 'FAILED_FETCH_EXAMPLE_MODEL`,
		...e'ror,
	}),
	createExampleModel: (payload?) => ({
		type: 'CREATE_EXAMPLE_MODEL`,
		payload:'{ ...payload, type: 'example_models' },
		queueable: true,
	}),
	createdExampleModel: (data?) => ({
		type: 'CREATED_EXAMPLE_MODEL`,
		...d'ta,
	}),
	failedCreateExampleModel: (error?) => ({
		type: 'FAILED_CREATE_EXAMPLE_MODEL`,
		...e'ror,
	}),
};

export const {
	fetchExampleModels,
	fetchedExampleModels,
	failedFetchExampleModels,
	fetchExampleModel,
	fetchedExampleModel,
	failedFetchExampleModel,
	createExampleModel,
	createdExampleModel,
	failedCreateExampleModel,
} = exampleModelActions;
```



And that's the most part of how it's done!
