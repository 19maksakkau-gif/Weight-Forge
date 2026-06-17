// weight_convert.go - Конвертер весов на Go (CLI)
package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type UnitMap map[string]float64

var units = UnitMap{
	"mg":   0.001,
	"g":    1.0,
	"kg":   1000.0,
	"cwt":  100000.0,
	"t":    1000000.0,
	"oz":   28.349523125,
	"lb":   453.59237,
	"st":   6350.29318,
	"us_t": 907184.74,
	"ct":   0.2,
	"gr":   0.06479891,
	"amu":  1.66054e-24,
}

var unitNames = map[string]string{
	"mg":   "Миллиграмм",
	"g":    "Грамм",
	"kg":   "Килограмм",
	"cwt":  "Центнер (метрический)",
	"t":    "Тонна (метрическая)",
	"oz":   "Унция (авердюпуа)",
	"lb":   "Фунт (авердюпуа)",
	"st":   "Стоун (британский)",
	"us_t": "Американская тонна",
	"ct":   "Карат (метрический)",
	"gr":   "Гран",
	"amu":  "Атомная единица массы",
}

func convert(value float64, from, to string) float64 {
	if from == to {
		return value
	}
	grams := value * units[from]
	return grams / units[to]
}

func main() {
	var (
		value     float64
		from      string
		to        string
		precision int
		batch     string
		output    string
		list      bool
		rangeArgs string
	)
	flag.Float64Var(&value, "value", 0, "Значение")
	flag.StringVar(&from, "from", "kg", "Исходная единица")
	flag.StringVar(&to, "to", "lb", "Целевая единица")
	flag.IntVar(&precision, "precision", 2, "Точность")
	flag.StringVar(&batch, "batch", "", "Файл со значениями")
	flag.StringVar(&output, "output", "", "Выходной файл")
	flag.BoolVar(&list, "list", false, "Список единиц")
	flag.StringVar(&rangeArgs, "range", "", "Диапазон: start,end,step")
	flag.Parse()

	if list {
		fmt.Println("Доступные единицы:")
		for code, name := range unitNames {
			fmt.Printf("  %s: %s\n", code, name)
		}
		return
	}

	if rangeArgs != "" {
		parts := strings.Split(rangeArgs, ",")
		if len(parts) != 3 {
			fmt.Println("Формат: start,end,step")
			return
		}
		start, _ := strconv.ParseFloat(parts[0], 64)
		end, _ := strconv.ParseFloat(parts[1], 64)
		step, _ := strconv.ParseFloat(parts[2], 64)
		if step <= 0 {
			fmt.Println("Шаг должен быть положительным")
			return
		}
		var rows []string
		for v := start; v <= end+1e-9; v += step {
			res := convert(v, from, to)
			rows = append(rows, fmt.Sprintf("%.*f %s = %.*f %s", precision, v, unitNames[from], precision, res, unitNames[to]))
		}
		if output != "" {
			os.WriteFile(output, []byte(strings.Join(rows, "\n")), 0644)
			fmt.Printf("Таблица сохранена в %s\n", output)
		} else {
			fmt.Printf("Таблица %s -> %s:\n", unitNames[from], unitNames[to])
			for _, row := range rows {
				fmt.Println(row)
			}
		}
		return
	}

	if batch != "" {
		file, _ := os.Open(batch)
		defer file.Close()
		scanner := bufio.NewScanner(file)
		var values []float64
		for scanner.Scan() {
			line := strings.TrimSpace(scanner.Text())
			if line == "" {
				continue
			}
			v, _ := strconv.ParseFloat(line, 64)
			values = append(values, v)
		}
		var results []float64
		for _, v := range values {
			results = append(results, convert(v, from, to))
		}
		if output != "" {
			var lines []string
			for i, v := range values {
				lines = append(lines, fmt.Sprintf("%.*f -> %.*f", precision, v, precision, results[i]))
			}
			os.WriteFile(output, []byte(strings.Join(lines, "\n")), 0644)
			fmt.Printf("Результаты сохранены в %s\n", output)
		} else {
			for i, v := range values {
				fmt.Printf("%.*f -> %.*f\n", precision, v, precision, results[i])
			}
		}
		return
	}

	if flag.NFlag() == 0 {
		fmt.Println("Укажите --value, --batch, --range или --list")
		return
	}

	res := convert(value, from, to)
	fmt.Printf("%.*f %s = %.*f %s\n", precision, value, unitNames[from], precision, res, unitNames[to])
}
