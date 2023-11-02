const FAIXAS_INSS = [
    { percent: 7.5, minValue: 0, maxValue: 1302, range: 1 },
    { percent: 9, minValue: 1302, maxValue: 2571.29, range: 2 },
    { percent: 12, minValue: 2571.3, maxValue: 3856.94, range: 3 },
    { percent: 14, minValue: 3856.94, maxValue: 7507.49, range: 4 },
    { percent: null, minValue: 7087.23, maxValue: 1 / 0, range: 5 },
]

const FAIXAS_IRRF = [
    { percent: 0, parcelTax: 0, minValue: 0, maxValue: 1903.98 },
    { percent: 7.5, parcelTax: 142.8, minValue: 1903.99, maxValue: 2826.65 },
    { percent: 15, parcelTax: 354.8, minValue: 2826.66, maxValue: 3751.05 },
    { percent: 22.5, parcelTax: 636.13, minValue: 3751.06, maxValue: 4664.68 },
    { percent: 27.5, parcelTax: 869.36, minValue: 4664.69, maxValue: null },
]

const DEDUCAO_DEPENDENTE = 189.59
const DESCONTO_INSS = 0.14 * FAIXAS_INSS[4].minValue - 163.82

export function salarioLiquido(salarioBruto: number, numeroDependentes: number = 0, desconto: number = 0) {
    const calculoInss = calcularINSS(salarioBruto)

    const calculoInssIrrf = calculateINSSandIRRF(salarioBruto, numeroDependentes)

    const totalDesconto = desconto + calculoInss.inssVal + calculoInssIrrf.irrfVal

    return {
        salarioLiquido: salarioBruto - totalDesconto,
        descontoTotal: totalDesconto,
        descontoInss: calculoInss.inssVal,
        descontoIrrf: calculoInssIrrf.irrfVal
    }
}

function calcularINSS(salarioBruto: number) {
    const faixaInss = FAIXAS_INSS.filter(t => t.minValue <= salarioBruto && salarioBruto <= t.maxValue)[0]

    const a = FAIXAS_INSS[0].maxValue * (FAIXAS_INSS[0].percent! / 100)
    const r = (FAIXAS_INSS[1].maxValue - FAIXAS_INSS[0].maxValue) * (FAIXAS_INSS[1].percent! / 100)
    const n = (FAIXAS_INSS[2].maxValue - FAIXAS_INSS[1].maxValue) * (FAIXAS_INSS[2].percent! / 100)
    let o = 0

    switch (faixaInss.range) {
        case 1:
            o = (faixaInss.percent! / 100) * salarioBruto;
            break;
        case 2:
            o = a + (salarioBruto - FAIXAS_INSS[0].maxValue) * (FAIXAS_INSS[1].percent! / 100);
            break;
        case 3:
            o = a + r + (salarioBruto - FAIXAS_INSS[1].maxValue) * (FAIXAS_INSS[2].percent! / 100);
            break;
        case 4:
            o = a + r + n + (salarioBruto - FAIXAS_INSS[2].maxValue) * (FAIXAS_INSS[3].percent! / 100);
            break;
        case 5:
            o = DESCONTO_INSS;
            break;
        default:
            o = 0;
    }

    const c = o !== DESCONTO_INSS ? +o.toFixed(2) / salarioBruto : null;
    return { inssVal: +o.toFixed(2), inssPorcent: +c! };
}

function calculateINSSandIRRF(salarioBruto: number, numeroDependentes: number) {
    const a = calcularINSS(salarioBruto)
    const r = a.inssVal
    const n = a.inssPorcent
    const o = FAIXAS_IRRF.filter(t => t.minValue < salarioBruto - r && (null === t.maxValue || t.maxValue > salarioBruto - r))[0]

    const c = salarioBruto - r - numeroDependentes * DEDUCAO_DEPENDENTE
    const l = (o && o.percent / 100) || 0
    const i = (o && o.parcelTax) || 0
    const u = Math.max(c * l - i, 0)
    const s = +(r + u).toFixed(2)
    const d = (u / c) * 100

    return {
        inssVal: r || 0,
        inssPorcent: n || 0,
        irrfVal: u || 0,
        irrfPorcent: l || 0,
        grossVal: +salarioBruto.toFixed(2),
        totalDiscount: s,
        liquidVal: salarioBruto - s,
        aliquotaIrrf: +d.toFixed(2)
    };
}