#!/usr/bin/env node
/**
 * weight_convert.js - Конвертер весов на JavaScript (Node.js CLI)
 */
const fs = require('fs');
const { program } = require('commander');

// Единицы (в граммах)
const UNITS = {
    mg: 0.001,
    g: 1.0,
    kg: 1000.0,
    cwt: 100000.0,
    t: 1000000.0,
    oz: 28.349523125,
    lb: 453.59237,
    st: 6350.29318,
    us_t: 907184.74,
    ct: 0.2,
    gr: 0.06479891,
    amu: 1.66054e-24,
};
const UNIT_NAMES = {
    mg: 'Миллиграмм',
    g: 'Грамм',
    kg: 'Килограмм',
    cwt: 'Центнер (метрический)',
    t: 'Тонна (метрическая)',
    oz: 'Унция (авердюпуа)',
    lb: 'Фунт (авердюпуа)',
    st: 'Стоун (британский)',
    us_t: 'Американская тонна',
    ct: 'Карат (метрический)',
    gr: 'Гран',
    amu: 'Атомная единица массы',
};

class WeightConverter {
    static convert(value, from, to) {
        if (from === to) return value;
        const grams = value * UNITS[from];
        return grams / UNITS[to];
    }

    static convertBatch(values, from, to) {
        return values.map(v => this.convert(v, from, to));
    }

    static generateTable(start, end, step, from, to, precision = 2) {
        const rows = [];
        for (let v = start; v <= end + 1e-9; v += step) {
            const res = this.convert(v, from, to);
            rows.push([
                `${v.toFixed(precision)} ${UNIT_NAMES[from]}`,
                `${res.toFixed(precision)} ${UNIT_NAMES[to]}`
            ]);
        }
        return rows;
    }
}

program
    .option('-v, --value <number>', 'Значение', parseFloat)
    .option('-f, --from <unit>', 'Исходная единица', 'kg')
    .option('-t, --to <unit>', 'Целевая единица', 'lb')
    .option('-p, --precision <number>', 'Точность', parseInt, 2)
    .option('-b, --batch <file>', 'Файл со значениями')
    .option('-o, --output <file>', 'Выходной файл')
    .option('--range <start,end,step>', 'Диапазон для таблицы')
    .option('--list', 'Показать список единиц')
    .parse(process.argv);

const opts = program.opts();

if (opts.list) {
    console.log('Доступные единицы:');
    for (const [code, name] of Object.entries(UNIT_NAMES)) {
        console.log(`  ${code}: ${name}`);
    }
    process.exit(0);
}

if (opts.range) {
    const [start, end, step] = opts.range.split(',').map(Number);
    if (step <= 0) { console.error('Шаг должен быть положительным'); process.exit(1); }
    const table = WeightConverter.generateTable(start, end, step, opts.from, opts.to, opts.precision);
    if (opts.output) {
        const content = table.map(row => `${row[0]} = ${row[1]}`).join('\n');
        fs.writeFileSync(opts.output, content, 'utf8');
        console.log(`Таблица сохранена в ${opts.output}`);
    } else {
        console.log(`Таблица ${UNIT_NAMES[opts.from]} -> ${UNIT_NAMES[opts.to]}:`);
        table.forEach(row => console.log(`${row[0]} = ${row[1]}`));
    }
    process.exit(0);
}

if (opts.batch) {
    const data = fs.readFileSync(opts.batch, 'utf8');
    const values = data.split('\n').filter(l => l.trim()).map(Number);
    const results = WeightConverter.convertBatch(values, opts.from, opts.to);
    if (opts.output) {
        const lines = values.map((v, i) => `${v} -> ${results[i]}`);
        fs.writeFileSync(opts.output, lines.join('\n'), 'utf8');
        console.log(`Результаты сохранены в ${opts.output}`);
    } else {
        values.forEach((v, i) => console.log(`${v} -> ${results[i].toFixed(opts.precision)}`));
    }
    process.exit(0);
}

if (opts.value !== undefined) {
    const result = WeightConverter.convert(opts.value, opts.from, opts.to);
    console.log(`${opts.value.toFixed(opts.precision)} ${UNIT_NAMES[opts.from]} = ${result.toFixed(opts.precision)} ${UNIT_NAMES[opts.to]}`);
} else {
    console.log('Укажите --value, --batch, --range или --list');
}
