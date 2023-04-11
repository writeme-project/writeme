<p align="center">
    <h1 align="center">
        {{icon}} {{name}}
    </h1>
    <p align="center">{{description}}</p>
</p>

<p align="center">
    {{shields}}
</p>

<div align="center">
    <h4>
        <a href="{{repository}}">
            👥 Contributing
        </a>
        <span>&nbsp;&nbsp;|&nbsp;&nbsp;</span>
        <a href="{{repository}}">
            🤝 Code of conduct
        </a>
        <span>&nbsp;&nbsp;|&nbsp;&nbsp;</span>
        <a href="{{repository}}/issues">
            🔎 Issues
        </a>
        <span>&nbsp;&nbsp;|&nbsp;&nbsp;</span>
    </h4>
</div>

| {{long_description}} |
| ------------------ |

## 📦 Packages

<table>
    <th>Package</th>
    <th>Version</th>
    <tbody>
        {{packages_td}}
    <tbody>
</table>

## 👥 Ways to contribute

## 🛠 Install

Clone this repository:

```bash
git clone {{repository}}
```

and install the dependencies:

```bash
cd {{name}} && {{pk_manager}}
```

## 📜 Usage

### Code quality and formatting

Run [ESLint](https://eslint.org/) to analyze the code and catch bugs:

```bash
{{pk_manager}} lint
```

Run [Prettier](https://prettier.io/) to check formatting rules:

```bash
{{pk_manager}} prettier
```

or to automatically format the code:

```bash
{{pk_manager}} prettier:write
```

### Conventional commits

{{name}} uses [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/). A [command line utility](https://github.com/commitizen/cz-cli) to commit using the correct syntax can be used by running:

```bash
{{pk_manager}} commit
```

It will also automatically check that the modified files comply with ESLint and Prettier rules.

### Testing

Test the code with coverage:

```bash
{{pk_manager}} {{cmd_test}}
```

### Run

Run the code:

```bash
{{pk_manager}} {{cmd_run}}
```

### Build

Build all the packages and compile contracts:

```bash
{{pk_manager}} {{cmd_build}}
```

A `dist` folder will be created inside each JavaScript package.

### Documentation

Generate a documentation website for each package:

```bash
{{pk_manager}} {{cmd_docs}}
```

The output will be placed on the `docs` folder.

## 👥 Authors && Contributors

{{author}}
{{contributors}}
