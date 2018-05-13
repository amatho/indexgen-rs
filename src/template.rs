const template: &str = #"
<!DOCTYPE html>
<html lang="en">
<head>
	<meta charset="UTF-8">
	<meta name="viewport" content="width=device-width, initial-scale=1.0">
	<meta http-equiv="X-UA-Compatible" content="ie=edge">
	<title>Index of {{dir}}</title>
	<style>
	body {
		margin: 0;
		padding: 0;
		box-sizing: border-box;
		font-family: sans-serif;
		line-height: 1.5;
		font-size: 62.5%;
	}
	table {
		width: 80%;
		font-size: 1.2rem;
		border-collapse: collapse;
		margin: 32px auto 0 auto;
	}
	table thead td {
		font-weight: 700;
		border-bottom: 2px solid #757575;
	}
	table tbody td {
		padding: 8px 4px;
	}
	table tbody .filetype {
		text-transform: capitalize;
	}
	</style>
</head>
<body>
	<table>
		<thead>
			<tr>
				<td>Filename</td>
				<td>Filetype</td>
			</tr>
		</thead>
		<tbody>
			{{#each file}}
			<tr>
				<td>
					<a href="{{this.name}}">{{this.name}}</a>
				</td>
				<td class="filetype">{{this.type}}</td>
			</tr>
			{{/each}}
		</tbody>
	</table>
</body>
</html>
"#;
